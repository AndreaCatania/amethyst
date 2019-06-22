

use crate::servers::PhysicsBodyTag;
use amethyst_core::ecs::{DenseVecStorage, Component};

/// PhysicsWorld is the opaque ID that identify a world in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsWorldTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsWorldTag{
    fn default() -> Self {
        panic!();
        PhysicsWorldTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsWorldTag{
    type Storage = DenseVecStorage<PhysicsWorldTag>;
}

/// The world server interface
/// This contains all functionalities to manipulate the world.
pub trait WorldServer{
    fn create_world(&mut self) -> PhysicsWorldTag;
    fn drop_world(&mut self, world: PhysicsWorldTag);

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag);

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32);
}
