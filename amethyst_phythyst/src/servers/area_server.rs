
use crate::objects::*;
use amethyst_core::components::Transform;
use nalgebra::{
    RealField,
};

/// This is the interface that contains all the functionalities that an area have.
///
/// The object that implement this interface is wrapped by the `AreaServer`.
pub trait AreaPhysicsServerTrait {
    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsAreaTag;

    fn drop_area(&mut self, area_tag: PhysicsAreaTag);
}

pub struct AreaDesc {
    pub shape: PhysicsShapeTag,
    pub transform: Transform,
}