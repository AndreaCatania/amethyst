use amethyst_core::components::Transform;
use nalgebra::{Isometry3, Point3, RealField, Vector3};

use crate::objects::*;

/// This is the interface used to manipulate the shapes
/// The object that implement this interface is implemented by `ShapePhysicsServer`.
/// It's stored as resource in the world.
pub trait ShapePhysicsServerTrait<N: crate::PtReal> {
    /// Create a shape and return the handle to it.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the shape is Dropped automatically.
    fn create_shape(&self, shape: &ShapeDesc<N>) -> PhysicsHandle<PhysicsShapeTag>;

    /// Change the internal shape description of this shape.
    fn update_shape(&self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>);
}

#[derive(Clone, Debug)]
pub enum ShapeDesc<N: crate::PtReal> {
    Sphere {
        radius: N,
    },
    Cube {
        half_extents: Vector3<N>,
    },
    Capsule {
        half_height: N,
        radius: N,
    },
    Cylinder {
        half_height: N,
        radius: N,
    },
    /// The plane is a shape with infinite size. The normal of the plane is Y+.
    /// Usually this shape is used as world margin.
    Plane,
    Convex {
        points: Vec<Point3<N>>,
    },
    TriMesh {
        points: Vec<Point3<N>>,
        indices: Vec<Point3<usize>>,
    },
    Compound {
        shapes: Vec<(Isometry3<N>, ShapeDesc<N>)>,
    },
}
