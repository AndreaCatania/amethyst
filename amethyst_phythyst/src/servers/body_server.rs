
use crate::objects::*;


/// This is the interface that contains all functionalities to manipulate
/// - RigidBody
/// - StaticBody
/// - KinematicBody
/// - Areas
///
/// The object that implement this interface is wrapped by `RBodyPhysicsServer`.
/// It's stored as resource in the world.
pub trait RBodyPhysicsServerTrait {

    fn create_body(&mut self) -> PhysicsBodyTag;
    fn drop_body(&mut self, body: PhysicsBodyTag);

}
