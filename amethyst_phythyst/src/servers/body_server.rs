use crate::objects::*;
use amethyst_core::components::Transform;
use nalgebra::{
    RealField,
    Vector3,
};

/// This is the interface that contains all functionalities to manipulate
/// - RigidBody
/// - StaticBody
/// - KinematicBody
///
/// The object that implement this interface is wrapped by `RBodyPhysicsServer`.
/// It's stored as resource in the world.
///
pub trait RBodyPhysicsServerTrait<N: RealField> {
    fn create_body(
        &mut self,
        world_tag: PhysicsWorldTag,
        body_desc: &RigidBodyDesc<N>,
    ) -> PhysicsBodyTag;

    fn drop_body(&mut self, body_tag: PhysicsBodyTag);

    /// Get the actual transformation of the body
    fn body_transform(&self, body_tag: PhysicsBodyTag) -> Transform;

    /// Clear forces
    fn clear_forces(&self, body: PhysicsBodyTag);

    /// Apply a central force to the body
    fn apply_force(&self, body: PhysicsBodyTag, force: &Vector3<N>);

    /// Apply central angular force to the body
    fn apply_torque(&self, body: PhysicsBodyTag, force: &Vector3<N>);

    /// Apply force at position to the body
    fn apply_force_at_position(&self, body: PhysicsBodyTag, force: &Vector3<N>, position: &Vector3<N>);

    /// Apply central impulse to the body
    fn apply_impulse(&self, body: PhysicsBodyTag, impulse: &Vector3<N>);

    /// Apply central angulat impulse to the body
    fn apply_angular_impulse(&self, body: PhysicsBodyTag, impulse: &Vector3<N>);

    /// Apply impulse at position to the body
    fn apply_impulse_at_position(&self, body: PhysicsBodyTag, impulse: &Vector3<N>, position: &Vector3<N>);

    /// Set the velocity of the body
    fn set_linear_velocity(&self, body: PhysicsBodyTag, velocity: &Vector3<N>);

    /// Get the velocity of the body
    fn linear_velocity(&self, body: PhysicsBodyTag) -> Vector3<N>;

    /// Set the angular velocity of the body
    fn set_angular_velocity(&self, body: PhysicsBodyTag, velocity: &Vector3<N>);

    /// Get the angular velocity of the body
    fn angular_velocity(&self, body: PhysicsBodyTag) -> Vector3<N>;
}

/// This structure holds all information about the Rigid body before it is created.
#[derive(Default)]
pub struct RigidBodyDesc<N> {
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
