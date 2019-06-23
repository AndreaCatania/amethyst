
use crate::servers_storage::{ServersStorageType,};

use amethyst_phythyst::{
    servers::{
        WorldPhysicsServerTrait,
    },
    objects::*,
};

use nphysics3d::{
    world::World
};

pub struct NWorldServer{
    storages: ServersStorageType,
}

impl NWorldServer{
    pub fn new(storages: ServersStorageType) -> NWorldServer {
        NWorldServer{
            storages,
        }
    }
}

impl WorldPhysicsServerTrait for NWorldServer{
    fn create_world(&mut self) -> PhysicsWorldTag {

        PhysicsWorldTag(self.storages.worlds_w().make_opaque(Box::new(World::new())))
    }

    fn drop_world(&mut self, world: PhysicsWorldTag){
        let mut w = self.storages.worlds_w();
        fail_cond!(!w.has(world.0));

        w.destroy(world.0);
    }

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag){

        let w = self.storages.worlds_w();

        let world = w.get(*world);
        fail_cond!(world.is_none());

        let world = world.unwrap();

        let r = self.storages.rbodies_r();
        let body = r.get(*body);
        fail_cond!(body.is_none());
        let body = body.unwrap();

        println!("Body a: {}", body.a);

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