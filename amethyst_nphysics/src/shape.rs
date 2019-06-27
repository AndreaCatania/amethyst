use amethyst_phythyst::{
    objects::*,
    servers::ShapeDesc,
};

use ncollide3d::shape::{
    ShapeHandle as NcShapeHandle,
    Ball as NcBall,
    Cuboid as NcCuboid,
    Plane as NcPlane,
    Cylinder as NcCylinder,
};

use nalgebra::{
    RealField,
    Vector3,
    convert,
    Unit,
};

pub struct RigidShape<N: RealField>{
    shape_desc: ShapeDesc<N>,
    shape_handle: NcShapeHandle<N>,
}

impl<N: RealField> RigidShape<N> {
    pub fn new(shape_desc: &ShapeDesc<N>) -> Self {
        RigidShape {
            shape_desc: shape_desc.clone(),
            shape_handle: RigidShape::generate_handle(shape_desc),
        }
    }

    pub fn update(&mut self, shape_desc: &ShapeDesc<N>){
        self.shape_desc = shape_desc.clone();
        self.shape_handle = RigidShape::generate_handle(shape_desc);

        // TODo please update the shape on all bodies
    }

    pub fn shape_handle(&self) -> &NcShapeHandle<N> {
        &self.shape_handle
    }
}

impl<N: RealField> RigidShape<N> {
    fn generate_handle(shape_desc: &ShapeDesc<N>) -> NcShapeHandle<N> {
        match shape_desc {
            ShapeDesc::Sphere{radius} => NcShapeHandle::new(NcBall::new(*radius)),
            ShapeDesc::Cube{half_extents} => NcShapeHandle::new(NcCuboid::new(*half_extents)),
            ShapeDesc::Plane => NcShapeHandle::new(NcPlane::new(Unit::new_normalize(Vector3::new(convert(0.0), convert(1.0), convert(0.0))))),
            //ShapeDesc::Cylinder{half_height, radius} => NcShapeHandle::new( NcCylinder::new(*half_height, *radius) ),
        }
    }
}