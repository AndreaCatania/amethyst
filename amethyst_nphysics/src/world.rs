use amethyst_phythyst::{objects::PhysicsBodyTag, PtReal};
use nphysics3d::world::{GeometricalWorld, MechanicalWorld};

use crate::{body_storage::BodyStorage, storage::StoreKey};

pub struct World<N: PtReal> {
    pub geometrical_world: GeometricalWorld<N, StoreKey, StoreKey>,
    pub mechanical_world: MechanicalWorld<N, BodyStorage<N>, StoreKey>,
}
