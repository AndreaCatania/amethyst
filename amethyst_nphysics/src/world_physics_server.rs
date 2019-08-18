use std::sync::RwLock;

use amethyst_phythyst::{
    objects::*,
    servers::{OverlapEvent, WorldPhysicsServerTrait},
    PtReal,
};
use nalgebra::Vector3;
use core::borrow::BorrowMut;
use nphysics3d::{
    utils::UserData as NpUserData,
    world::{GeometricalWorld, MechanicalWorld},
};
use ncollide3d::query::Proximity;

use crate::{
    conversors::*,
    servers_storage::{ServersStorageType, BodiesStorageWrite, CollidersStorageWrite},
    body::BodyData,
    utils::*,
    AreaNpServer,
    RBodyNpServer,
    ShapeNpServer,
    storage::StoreKey,
    body_storage::BodyStorage,
};

pub struct WorldNpServer<N: PtReal> {
    pub storages: ServersStorageType<N>,
    pub geometrical_world: RwLock<GeometricalWorld<N, StoreKey, StoreKey>>,
    pub mechanical_world: RwLock<MechanicalWorld<N, BodyStorage<N>, StoreKey>>,
}

impl<N: PtReal> WorldNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> WorldNpServer<N> {
        WorldNpServer {
            storages,
            geometrical_world: RwLock::new(GeometricalWorld::new()),
            mechanical_world: RwLock::new(MechanicalWorld::new(Vector3::new(
                N::from(0.0),
                N::from(-9.8),
                N::from(0.0),
            ))),
        }
    }
}

impl<N: PtReal> WorldNpServer<N> {
    fn garbage_collect(&self) {
        let mut gc = self.storages.gc.write().unwrap();
        let mut bodies_storage = self.storages.bodies_w();
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
    }

    fn fetch_events(
        g_world: &mut GeometricalWorld<N, StoreKey, StoreKey>,
        m_world: &mut MechanicalWorld<N, BodyStorage<N>, StoreKey>,
        bodies: &mut BodiesStorageWrite<N>,
        colliders: &mut CollidersStorageWrite<N>) {

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
            let events = g_world.proximity_events();
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

    fn step(&self, delta_time: N) {
        self.garbage_collect();

        let mut mw = self.mechanical_world.write().unwrap();
        let mut gw = self.geometrical_world.write().unwrap();

        let mut bodies = self.storages.bodies_w();
        let mut colliders = self.storages.colliders_w();
        let mut joints = self.storages.joints_w();
        let mut force_generator = self.storages.force_generator_w();

        //// TODO this is not completely free. So perform it only when needed.
        mw.set_timestep(delta_time);
        mw.step(
            &mut *gw,
            &mut *bodies,
            &mut *colliders,
            &mut *joints,
            &mut *force_generator,
        );

        Self::fetch_events(&mut *gw, &mut *mw, &mut bodies, &mut colliders);
    }
}
