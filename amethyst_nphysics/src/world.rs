use nphysics3d::world::{
    GeometricalWorld,
    MechanicalWorld,
};
use amethyst_phythyst::{
    objects::PhysicsBodyTag,
    PtReal,
};

use crate::{
    storage::StoreKey,
    body_storage::BodyStorage
};

pub struct World<N: PtReal>{
    pub(crate) geometrical_world: GeometricalWorld<N, StoreKey, StoreKey>,
    pub(crate) mechanical_world: MechanicalWorld<N, BodyStorage<N>, StoreKey>,
}
