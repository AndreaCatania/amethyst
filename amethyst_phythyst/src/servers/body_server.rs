
use crate::servers::PhysicsWorldTag;

/// PhysicsBody is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsBodyTag(pub std::num::NonZeroUsize);

/// The body server interface
/// This contains all functionalities to manipulate
/// - RigidBody
/// - StaticBody
/// - KinematicBody
/// - Areas
///
pub trait RigidBodyServer {

    fn create_body(&mut self) -> PhysicsBodyTag;
    fn drop_body(&mut self, body: PhysicsBodyTag);

}