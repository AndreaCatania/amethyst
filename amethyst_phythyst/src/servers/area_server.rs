
use crate::objects::*;
use amethyst_core::components::Transform;
use nalgebra::{
    RealField,
};

/// This is the interface that contains all the functionalities that an area have.
///
/// The object that implement this interface is wrapped by the `AreaServer`.
pub trait AreaPhysicsServerTrait {
    /// Create an Area and return its handle.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the Area is Dropped automatically.
    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsHandle<PhysicsAreaTag>;

    /// Returns the list of events occurred in the last step.
    /// Is mandatory check this array each sub step to be sure to not miss any event.
    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent>;

}

pub struct AreaDesc {
    pub shape: PhysicsShapeTag,
    pub transform: Transform,
}

#[derive(Copy, Clone, Debug)]
pub enum OverlapEvent{
    Enter(PhysicsBodyTag),
    Exit(PhysicsBodyTag),
}