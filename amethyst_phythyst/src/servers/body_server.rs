
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
pub trait RBodyPhysicsServerTrait<N> {

    fn create_body(&mut self, world_tag: PhysicsWorldTag, body_desc : &RigidBodyDesc<N>) -> PhysicsBodyTag;
    fn drop_body(&mut self, body_tag: PhysicsBodyTag);

    fn body_transform(&self, body_tag: PhysicsBodyTag) -> Transform;
}

/// This structure holds all information about the Rigid body before it is created.
#[derive(Default)]
pub struct RigidBodyDesc<N>{
    pub mode: BodyMode,
    pub shape: PhysicsShapeTag,
    pub transformation: Transform,
    pub mass: N,
}

/// The mode of a body.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BodyMode {
    /// The body is disabled and ignored by the physics engine.
    Disabled,
    /// The body is static and thus cannot move.
    Static,
    /// The body is dynamic and thus can move and is subject to forces.
    Dynamic,
    /// The body is kinematic so its velocity is controlled by the user and it is not affected by forces and constraints.
    Kinematic,
}

impl Default for BodyMode {
    fn default() -> Self {
        BodyMode::Dynamic
    }
}

