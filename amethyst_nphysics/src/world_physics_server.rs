
use amethyst_phythyst::{
    servers::{
        WorldPhysicsServerTrait,
    },
    objects::*,
};

use nalgebra::Vector3;

use core::borrow::BorrowMut;

use crate::{
    world::World,
    servers_storage::{
        ServersStorageType,
    }
};

pub struct WorldNpServer {
    storages: ServersStorageType,
}

impl WorldNpServer {
    pub fn new(storages: ServersStorageType) -> WorldNpServer {
        WorldNpServer {
            storages,
        }
    }
}

impl WorldPhysicsServerTrait for WorldNpServer {
    fn create_world(&mut self) -> PhysicsWorldTag {

        let mut w = World::new();

        w.set_gravity(Vector3::new(0.0, -9.8, 0.0));

        PhysicsWorldTag(self.storages.worlds_w().make_opaque(Box::new(w)))
    }

    fn drop_world(&mut self, world: PhysicsWorldTag){
        let mut w = self.storages.worlds_w();
        fail_cond!(!w.has(world.0));

        w.destroy(world.0);
    }

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag){

        let mut w = self.storages.worlds_w();

        let world = w.get_mut(*world);
        fail_cond!(world.is_none());
        let mut world = world.unwrap();

        let mut r = self.storages.rbodies_w();
        let body = r.get_mut(*body);
        fail_cond!(body.is_none());
        let mut body = body.unwrap();

        body.set_world(world);
    }

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32){
        let mut w = self.storages.worlds_w();
        let world = w.get_mut(world.0);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}