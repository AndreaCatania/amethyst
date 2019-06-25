
use crate::objects::*;
use amethyst_core::components::Transform;

/// This is the interface that contains all functionalities to manipulate
/// - RigidBody
/// - StaticBody
/// - KinematicBody
/// - Areas
///
/// The object that implement this interface is wrapped by `RBodyPhysicsServer`.
/// It's stored as resource in the world.
pub trait RBodyPhysicsServerTrait {

    fn create_body(&mut self, world_tag: PhysicsWorldTag, body_desc : &RigidBodyDesc) -> PhysicsBodyTag;
    fn drop_body(&mut self, body_tag: PhysicsBodyTag);

    fn body_transform(&self, body_tag: PhysicsBodyTag) -> Transform;
}

/// This structure holds all information about the Rigid body before it is created.
#[derive(Default)]
pub struct RigidBodyDesc{
    pub transformation: Transform,
    pub mass: f32,
}