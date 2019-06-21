
use crate::storages::Storages;
use crate::storage::Storage;
use std::{
    rc::Rc,
    cell::RefCell
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
    storages: Rc<RefCell<Storages>>,
}

impl NWorldServer{
    pub fn new(storages: Rc<RefCell<Storages>>) -> NWorldServer {
        NWorldServer{
            storages,
        }
    }
}

impl WorldServer for NWorldServer{
    fn create(&mut self) -> PhysicsWorldTag {
        let w = self.storages.borrow_mut();
        PhysicsWorldTag(w.worlds.make_opaque(Box::new(World::new())))
    }

    fn drop(&mut self, world: PhysicsWorldTag){
        fail_cond!(!self.storages.borrow_mut().worlds.has(world.0));

        self.storages.borrow_mut().worlds.drop(world.0);
    }

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag){

    }

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32){
        let world = self.storages.borrow_mut().worlds.get_mut(world.0);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}