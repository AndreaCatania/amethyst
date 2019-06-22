
use crate::servers::PhysicsWorldTag;
use amethyst_core::ecs::{DenseVecStorage, Component};

/// PhysicsBody is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsBodyTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsBodyTag{
    fn default() -> Self {
        panic!();
        PhysicsBodyTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsBodyTag{
    type Storage = DenseVecStorage<PhysicsBodyTag>;
}

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