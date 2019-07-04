use amethyst_phythyst::{objects::*, servers::{
    WorldPhysicsServerTrait,
    OverlapEvent,
}};

use nalgebra::{RealField, Vector3};

use core::borrow::BorrowMut;

use crate::{utils::*, servers_storage::ServersStorageType, world::World};

use nphysics3d::{
    world::World as NpWorld,
    utils::UserData as NpUserData,
};

use ncollide3d::{
    query::Proximity,
};

pub struct WorldNpServer<N: RealField> {
    storages: ServersStorageType<N>,
}

impl<N: RealField> WorldNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> WorldNpServer<N> {
        WorldNpServer { storages }
    }
}

impl<N: RealField> WorldNpServer<N> {
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
                    match e.prev_status{
                        Proximity::Intersecting => {
                            continue;
                        }
                        _ => {
                            0 // Enter
                        }
                    }
                }
                _ => {
                    match e.prev_status{
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

            let body_1_ud = collider1.user_data().unwrap().downcast_ref::<UserData>().unwrap();
            let body_2_ud = collider2.user_data().unwrap().downcast_ref::<UserData>().unwrap();

            let mut area_tag;
            let mut body_tag;

            match body_1_ud.object_type() {
                ObjectType::Area => {
                    area_tag = body_1_ud.store_tag();
                    body_tag = body_2_ud.store_tag();
                }
                _ => {
                    area_tag = body_2_ud.store_tag();
                    body_tag = body_1_ud.store_tag();
                }
            }

            let area = s.get_mut(area_tag).unwrap();

            if status == 0 {
                // Enter
                area.overlap_events.push(OverlapEvent::Enter(PhysicsBodyTag(body_tag)));
            }else{
                // Exit
                area.overlap_events.push(OverlapEvent::Exit(PhysicsBodyTag(body_tag)));
            }
        }
    }
}

impl<N: RealField> WorldPhysicsServerTrait<N> for WorldNpServer<N> {
    fn create_world(&mut self) -> PhysicsHandle<PhysicsWorldTag> {
        let mut w = World::<N>::new();

        w.set_gravity(Vector3::new(
            nalgebra::convert(0.0),
            nalgebra::convert(-9.8),
            nalgebra::convert(0.0),
        ));

        PhysicsHandle::new(PhysicsWorldTag(self.storages.worlds_w().make_opaque(Box::new(w))), self.storages.gc.clone())
    }

    fn drop_world(&mut self, world: PhysicsWorldTag) {
        let mut w = self.storages.worlds_w();
        fail_cond!(!w.has(world.0));

        w.destroy(world.0);
    }

    fn step(&self, world: PhysicsWorldTag, delta_time: N) {
        let mut w = self.storages.worlds_w();
        let world = w.get_mut(world.0);
        fail_cond!(world.is_none());
        let mut world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();

        self.fetch_events(world);
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
