
use crate::objects::*;
use amethyst_core::components::Transform;

/// This is the interface used to manipulate the shapes
/// The object that implement this interface is implemented by `ShapePhysicsServer`.
/// It's stored as resource in the world.
pub trait ShapePhysicsServerTrait{
   fn create_shape(&mut self);
   fn drop_shape(&mut self);

   fn update_shape(&mut self);
}