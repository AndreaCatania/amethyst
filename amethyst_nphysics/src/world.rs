use nphysics3d::world::{
    GeometricalWorld,
    MechanicalWorld,
};
use amethyst_phythyst::{
    objects::PhysicsBodyTag,
    PtReal,
};

pub struct World<N: PtReal>{
    pub(crate) geometrical_world: GeometricalWorld<N, usize, usize>,
    //pub(crate) mechanical_world: MechanicalWorld<N, StoreTag,>,
}
