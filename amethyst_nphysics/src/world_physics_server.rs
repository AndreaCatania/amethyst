use amethyst_phythyst::{
    objects::*,
    servers::{OverlapEvent, WorldPhysicsServerTrait},
    PhysicsReal,
};

use nalgebra::{Vector3};

use core::borrow::BorrowMut;

use crate::{
    servers_storage::{ServersStorageType, WorldStorageWrite},
    utils::*,
    world::World,
    AreaNpServer, RBodyNpServer, ShapeNpServer,
};

use nphysics3d::{utils::UserData as NpUserData, world::World as NpWorld};

use ncollide3d::query::Proximity;

pub struct WorldNpServer<N: PhysicsReal> {
    storages: ServersStorageType<N>,
}

impl<N: PhysicsReal> WorldNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> WorldNpServer<N> {
        WorldNpServer { storages }
    }

    fn drop_world(world_tag: PhysicsWorldTag, worlds_storage: &mut WorldStorageWrite<N>) {
        // Here should be check if there are active bodies, but how?

        worlds_storage.destroy(*world_tag);
    }
}

impl<N: PhysicsReal> WorldNpServer<N> {
    fn garbage_collect(&self) {
        let mut gc = self.storages.gc.write().unwrap();
        let mut worlds_storage = self.storages.worlds_w();
        let mut rbodies_storage = self.storages.rbodies_w();
        let mut areas_storage = self.storages.areas_w();
        let mut shapes_storage = self.storages.shapes_w();

        {
            for rb in gc.bodies.iter() {
                RBodyNpServer::drop_body(
                    *rb,
                    &mut worlds_storage,
                    &mut rbodies_storage,
                    &mut shapes_storage,
                );
            }

            // The body drop can never fail.
            gc.bodies.clear();
        }

        {
            for rb in gc.bodies.iter() {
                RBodyNpServer::drop_body(
                    *rb,
                    &mut worlds_storage,
                    &mut rbodies_storage,
                    &mut shapes_storage,
                );
            }

            // The body drop can never fail.
            gc.bodies.clear();
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
                // Clear the garbage collector
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

    fn fetch_events(&self, world: &mut NpWorld<N>) {
        let mut s = self.storages.areas_w();

        let events = world.proximity_events();
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

            let collider1 = world.collider(e.collider1).unwrap();
            let collider2 = world.collider(e.collider2).unwrap();

            let body_1_ud = collider1
                .user_data()
                .unwrap()
                .downcast_ref::<UserData>()
                .unwrap();
            let body_2_ud = collider2
                .user_data()
                .unwrap()
                .downcast_ref::<UserData>()
                .unwrap();

            let mut area_tag;
            let mut body_tag;
            let mut body_entity;

            match body_1_ud.object_type() {
                ObjectType::Area => {
                    area_tag = body_1_ud.store_tag();
                    body_tag = body_2_ud.store_tag();
                    body_entity = body_2_ud.entity();
                }
                _ => {
                    area_tag = body_2_ud.store_tag();
                    body_tag = body_1_ud.store_tag();
                    body_entity = body_1_ud.entity();
                }
            }

            let area = s.get_mut(area_tag).unwrap();

            if status == 0 {
                // Enter
                area.overlap_events
                    .push(OverlapEvent::Enter(PhysicsBodyTag(body_tag), body_entity));
            } else {
                // Exit
                area.overlap_events
                    .push(OverlapEvent::Exit(PhysicsBodyTag(body_tag), body_entity));
            }
        }
    }
}

impl<N: PhysicsReal> WorldPhysicsServerTrait<N> for WorldNpServer<N> {
    fn create_world(&mut self) -> PhysicsHandle<PhysicsWorldTag> {
        let mut w = World::<N>::new();

        w.set_gravity(Vector3::new(
            nalgebra::convert(0.0),
            nalgebra::convert(-9.8),
            nalgebra::convert(0.0),
        ));

        PhysicsHandle::new(
            PhysicsWorldTag(self.storages.worlds_w().make_opaque(Box::new(w))),
            self.storages.gc.clone(),
        )
    }

    fn step(&self, world: PhysicsWorldTag, delta_time: N) {
        self.garbage_collect();

        {
            let mut w = self.storages.worlds_w();
            let world = w.get_mut(world.0);
            fail_cond!(world.is_none());
            let mut world = world.unwrap();

            world.set_timestep(delta_time);
            world.step();

            self.fetch_events(world);
        }
    }

    fn consume_events(&self) {
        // READ BEFORE REMOVE
        //
        // Is important to perform this operation inside the stepping once this is removed
        let mut s = self.storages.areas_w();
        for a in s.iter_mut() {
            a.1.overlap_events.clear();
        }
    }
}
