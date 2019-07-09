
use crate::objects::*;
use amethyst_core::{
    ecs::Entity,
    Float,
};
use nalgebra::{
    RealField,
    Isometry3,
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

    /// Set the entity which holds this body.
    fn set_entity(&self, area_tag: PhysicsAreaTag, index: Option<Entity>);

    /// Get the entity which holds this body.
    /// This returns Some only if the entity was associated during its creation.
    ///
    /// All the physical APIs events returns the PhysicalTag, using this function
    /// is possible to retrieve the Entity index and perform some operation in SPECS style.
    fn entity(&self, area_tag: PhysicsAreaTag ) -> Option<Entity>;

    /// Set the transformation of the area
    fn set_body_transform(&self, area: PhysicsAreaTag, transf: &Isometry3<Float>);

    // TODO pelase return an iterator and avoid to copy vectors around
    /// Returns the list of events occurred in the last step.
    /// Is mandatory check this array each sub step to be sure to not miss any event.
    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent>;
}

pub struct AreaDesc {
    pub shape: PhysicsShapeTag,
}

#[derive(Copy, Clone, Debug)]
pub enum OverlapEvent{
    Enter(PhysicsBodyTag, Option<Entity>),
    Exit(PhysicsBodyTag, Option<Entity>),
}