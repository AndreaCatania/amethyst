
use amethyst_phythyst::{
    servers::{
        WorldPhysicsServerTrait,
    },
    objects::*,
};

use nalgebra::{
    RealField,
    Vector3,
};

use core::borrow::BorrowMut;

use crate::{
    world::World,
    servers_storage::{
        ServersStorageType,
    }
};

pub struct WorldNpServer<N: RealField> {
    storages: ServersStorageType<N>,
}

impl<N: RealField> WorldNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> WorldNpServer<N> {
        WorldNpServer {
            storages,
        }
    }
}

impl<N: RealField> WorldPhysicsServerTrait<N> for WorldNpServer<N> {
    fn create_world(&mut self) -> PhysicsWorldTag {

        let mut w = World::<N>::new();

        w.set_gravity(Vector3::new(
            nalgebra::convert(0.0),
            nalgebra::convert(-9.8),
            nalgebra::convert(0.0)));

        PhysicsWorldTag(self.storages.worlds_w().make_opaque(Box::new(w)))
    }

    fn drop_world(&mut self, world: PhysicsWorldTag){
        let mut w = self.storages.worlds_w();
        fail_cond!(!w.has(world.0));

        w.destroy(world.0);
    }

    fn step(&mut self, world: PhysicsWorldTag, delta_time: N){
        let mut w = self.storages.worlds_w();
        let world = w.get_mut(world.0);
        fail_cond!(world.is_none());
        let world = world.unwrap();

        world.set_timestep(delta_time);
        world.step();
    }
}