use amethyst_phythyst::servers::{
    WorldHandle, WorldServer, Memory
};

use nphysics3d::{
    world::World
};

pub struct NWorldServer{
    world_memory: Memory<World<f32>>,
}

impl NWorldServer{
    pub fn new() -> NWorldServer {
        NWorldServer{
            world_memory: Memory::new(),
        }
    }
}

impl WorldServer for NWorldServer{
    fn create_world(&self) -> WorldHandle {
        
        let mut world : World<f32> = World::new();

        WorldHandle(std::num::NonZeroUsize::new(1).unwrap())
    }
}