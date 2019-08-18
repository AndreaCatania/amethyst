use amethyst_phythyst::{
    objects::*,
    servers::{OverlapEvent, WorldPhysicsServerTrait},
    PtReal,
};

use nalgebra::Vector3;

use core::borrow::BorrowMut;

use crate::{
    conversors::*,
    servers_storage::{ServersStorageType, WorldStorageWrite},
    rigid_body::BodyData,
    utils::*,
    world::World,
    AreaNpServer,
    RBodyNpServer,
    ShapeNpServer,
};

use nphysics3d::{
    utils::UserData as NpUserData,
    world::{GeometricalWorld, MechanicalWorld},
};

use ncollide3d::query::Proximity;

pub struct WorldNpServer<N: PtReal> {
    storages: ServersStorageType<N>,
}

impl<N: PtReal> WorldNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> WorldNpServer<N> {
        WorldNpServer { storages }
    }

    fn drop_world(world_tag: PhysicsWorldTag, worlds_storage: &mut WorldStorageWrite<N>) {
        worlds_storage.destroy(tag_to_store_key(world_tag.0));
    }
}

impl<N: PtReal> WorldNpServer<N> {
    fn garbage_collect(&self) {
        let mut gc = self.storages.gc.write().unwrap();
        let mut worlds_storage = self.storages.worlds_w();
        let mut bodies_storage = self.storages.rbodies_w();
        let mut colliders_storage = self.storages.colliders_w();
        let mut shapes_storage = self.storages.shapes_w();

        {
            for rb in gc.bodies.iter() {
                RBodyNpServer::drop_body(
                    *rb,
                    &mut bodies_storage,
                    &mut colliders_storage,
                    &mut shapes_storage,
                );
            }

            gc.bodies.clear();
        }

        {
            for area in gc.areas.iter() {
                AreaNpServer::drop_area(
                    *area,
                    &mut bodies_storage,
                    &mut colliders_storage,
                    &mut shapes_storage,
                );
            }

            gc.areas.clear();
        }

        // This happen after the bodies and the areas since they depend on this.
        {
            // Not all shapes can be safely removed since they could be assigned to Rigid Body and Areas.
            // If a shape is not removed it remains in the garbage collector.
            let mut removed_shape = Vec::<PhysicsShapeTag>::with_capacity(gc.shapes.len());

            for s in gc.shapes.iter() {
                if ShapeNpServer::drop_shape(*s, &mut shapes_storage) {
                    removed_shape.push(*s);
                }
            }

            if removed_shape.len() > 0 {
                // Remove from GC only the removed shapes.
                gc.shapes.retain(|&s| !removed_shape.contains(&s));
            }
        }

        {
            for w in gc.worlds.iter() {
                WorldNpServer::drop_world(*w, &mut worlds_storage);
            }

            gc.worlds.clear();
        }
    }

    fn fetch_events(&self, world: &mut World<N>) {
        let mut bodies = self.storages.rbodies_w();
        let mut colliders = self.storages.colliders_w();

        // Clear old events
        for (i, b) in bodies.iter_mut() {
            match &mut b.body_data {
                BodyData::Area(e) => {
                    e.clear();
                },
                _ => {}
            }
        }

        {
            // Fetch new events
            let events = world.geometrical_world.proximity_events();
            for e in events {
                if e.prev_status == e.new_status {
                    continue;
                }

                // 0 Enter, 1 Exit
                let status = match e.new_status {
                    Proximity::Intersecting => {
                        match e.prev_status {
                            Proximity::Intersecting => {
                                continue;
                            }
                            _ => {
                                0 // Enter
                            }
                        }
                    }
                    _ => {
                        match e.prev_status {
                            Proximity::Intersecting => {
                                1 // Exit
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                };

                let collider1 = colliders.get_collider(e.collider1).unwrap();
                let collider2 = colliders.get_collider(e.collider2).unwrap();

                let body_1_ud: &UserData = collider1
                    .user_data()
                    .unwrap()
                    .downcast_ref::<UserData>()
                    .unwrap();
                let body_2_ud: &UserData = collider2
                    .user_data()
                    .unwrap()
                    .downcast_ref::<UserData>()
                    .unwrap();

                let (area_tag, body_key, body_entity) = match body_1_ud.object_type() {
                    ObjectType::RigidBody => {
                        (
                            body_2_ud.store_key(),
                            body_1_ud.store_key(),
                            body_1_ud.entity()
                        )
                    }
                    ObjectType::Area => {
                        (
                            body_1_ud.store_key(),
                            body_2_ud.store_key(),
                            body_2_ud.entity()
                        )
                    }
                };

                let area = bodies.get_body_mut(area_tag).unwrap();
                if let BodyData::Area(e) = &mut area.body_data{
                    if status == 0 {
                        // Enter
                        e.push(OverlapEvent::Enter(PhysicsBodyTag(store_key_to_tag(body_key)), body_entity));
                    } else {
                        // Exit
                        e.push(OverlapEvent::Exit(PhysicsBodyTag(store_key_to_tag(body_key)), body_entity));
                    }
                }
            }
        }
    }
}

impl<N: PtReal> WorldPhysicsServerTrait<N> for WorldNpServer<N> {
    fn create_world(&mut self) -> PhysicsHandle<PhysicsWorldTag> {
        let mut w = World::<N> {
            geometrical_world: GeometricalWorld::new(),
            mechanical_world: MechanicalWorld::new(Vector3::new(
                N::from(0.0),
                N::from(-9.8),
                N::from(0.0),
            )),
        };

        PhysicsHandle::new(
            PhysicsWorldTag(store_key_to_tag(
                self.storages.worlds_w().make_opaque(Box::new(w)),
            )),
            self.storages.gc.clone(),
        )
    }

    fn step(&self, world_tag: PhysicsWorldTag, delta_time: N) {
        let world_key = tag_to_store_key(world_tag.0);
        self.garbage_collect();

        let mut w = self.storages.worlds_w();
        let world = w.get_mut(world_key);
        fail_cond!(world.is_none());
        let mut world = world.unwrap();

        let mut bodies = self.storages.rbodies_w();
        let mut colliders = self.storages.colliders_w();
        let mut joints = self.storages.joints_w();
        let mut force_generator = self.storages.force_generator_w();

        //// TODO this is not completely free. So perform it only when needed.
        world.mechanical_world.set_timestep(delta_time);
        world.mechanical_world.step(
            &mut world.geometrical_world,
            &mut *bodies,
            &mut *colliders,
            &mut *joints,
            &mut *force_generator,
        );

        self.fetch_events(world);
    }
}
