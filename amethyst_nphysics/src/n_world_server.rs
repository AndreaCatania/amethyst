
use crate::storage::Storage;

use amethyst_phythyst::{
    StoreTag,
    servers::{
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
    fn create(&mut self) -> StoreTag {
        self.world_storage.make_opaque(Box::new(World::new()))
    }

    fn drop(&mut self, world: StoreTag){
        fail_cond!(!self.world_storage.has(world));

        self.world_storage.drop(world);
    }

    fn step(&mut self, world: StoreTag, delta_time: f32){
        let world = self.world_storage.get_mut(world);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}