
use crate::storages::Storages;
use crate::storage::Storage;
use std::sync::{
    Arc, RwLock,
};
use amethyst_phythyst::{
    servers::{
        PhysicsWorldTag,
        PhysicsBodyTag,
        WorldServer,
    }
};

use nphysics3d::{
    world::World
};

pub struct NWorldServer{
    storages: Arc<RwLock<Storages>>,
}

impl NWorldServer{
    pub fn new(storages: Arc<RwLock<Storages>>) -> NWorldServer {
        NWorldServer{
            storages,
        }
    }
}

impl WorldServer for NWorldServer{
    fn create_world(&mut self) -> PhysicsWorldTag {
        PhysicsWorldTag(storage_write!(self.storages).worlds.make_opaque(Box::new(World::new())))
    }

    fn drop_world(&mut self, world: PhysicsWorldTag){
        let mut s = storage_write!(self.storages);
        fail_cond!(!s.worlds.has(world.0));

        s.worlds.drop(world.0);
    }

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag){

    }

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32){
        let mut s = storage_write!(self.storages);
        let world = s.worlds.get_mut(world.0);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}