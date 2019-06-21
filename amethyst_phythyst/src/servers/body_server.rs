
use crate::servers::PhysicsWorldTag;

/// PhysicsBody is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsBodyTag(pub std::num::NonZeroUsize);

/// The body server interface
/// This contains all functionalities to manipulate the body.
pub trait BodyServer{

    fn create(&mut self, world: PhysicsWorldTag) -> PhysicsBodyTag;
    fn drop(&mut self, body: PhysicsBodyTag);
}