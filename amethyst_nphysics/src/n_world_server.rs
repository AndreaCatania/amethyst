
use crate::storage::Storage;

use amethyst_phythyst::{
    servers::{
        PhysicsWorldTag,
        WorldServer,
    }
};

use nphysics3d::{
    world::World
};

pub struct NWorldServer{
    world_storage: Storage<Box<World<f32>>>,
}

impl NWorldServer{
    pub fn new() -> NWorldServer {
        NWorldServer{
            world_storage: Storage::new(1, 1),
        }
    }
}

impl WorldServer for NWorldServer{
    fn create(&mut self) -> PhysicsWorldTag {
        PhysicsWorldTag(self.world_storage.make_opaque(Box::new(World::new())))
    }

    fn drop(&mut self, world: PhysicsWorldTag){
        fail_cond!(!self.world_storage.has(world.0));

        self.world_storage.drop(world.0);
    }

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32){
        let world = self.world_storage.get_mut(world.0);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}