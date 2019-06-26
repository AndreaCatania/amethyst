use amethyst_phythyst::{
    servers::{
        ShapePhysicsServerTrait,
        ShapeDesc,
    },
    objects::*,
};

use crate::{
    servers_storage::*,
    conversors::*,
    shape::RigidShape,
};

use nalgebra::RealField;

pub struct ShapeNpServer<N: RealField>{
    storages: ServersStorageType<N>,
}

impl<N: RealField> ShapeNpServer<N>{
    pub fn new(storages: ServersStorageType<N>) -> Self {
        ShapeNpServer {
            storages,
        }
    }
}

impl<N: RealField> ShapePhysicsServerTrait<N> for ShapeNpServer<N>{
    fn create_shape(&mut self, shape: &ShapeDesc<N>) -> PhysicsShapeTag {

        PhysicsShapeTag(self.storages.shapes_w().make_opaque(Box::new(RigidShape::new(shape))))
    }

    fn drop_shape(&mut self, shape_tag: PhysicsShapeTag){
        unimplemented!();
    }

    fn update_shape(&mut self, shape_tag: PhysicsShapeTag, shape: &ShapeDesc<N>){
        unimplemented!();
    }
}

