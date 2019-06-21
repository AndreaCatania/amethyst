

use crate::servers::PhysicsBodyTag;

/// PhysicsWorld is the opaque ID that identify a world in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsWorldTag(pub std::num::NonZeroUsize);

/// The world server interface
/// This contains all functionalities to manipulate the world.
pub trait WorldServer{
    fn create(&mut self) -> PhysicsWorldTag;
    fn drop(&mut self, world: PhysicsWorldTag);

    fn add_body(&mut self, world: PhysicsWorldTag, body: PhysicsBodyTag);

    fn step(&mut self, world: PhysicsWorldTag, delta_time: f32);
}
