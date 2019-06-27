
use crate::objects::*;
use amethyst_core::{
   components::Transform,
};
use nalgebra::{
   Vector3,
   RealField
};

/// This is the interface used to manipulate the shapes
/// The object that implement this interface is implemented by `ShapePhysicsServer`.
/// It's stored as resource in the world.
pub trait ShapePhysicsServerTrait<N: RealField>{
   fn create_shape(&mut self, shape: &ShapeDesc<N>) -> PhysicsShapeTag;
   fn drop_shape(&mut self, shape_tag: PhysicsShapeTag);

   fn update_shape(&mut self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>);
}

#[derive(Clone)]
pub enum ShapeDesc<N: RealField> {
   Sphere{radius: N},
   Cube{half_extents: Vector3<N>},
   /// The plane is a shape with infinite size. The normal of the plane is Y+.
   /// Usually this shape is used as world margin.
   Plane,
   //Cylinder{half_height: N, radius: N},
}