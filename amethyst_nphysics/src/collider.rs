use nphysics3d::object::{
    BodyHandle as NpBodyHandle,
    Collider as NpCollider
};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal};

use crate::storage::StoreKey;

pub struct Collider<N: PtReal, BH: NpBodyHandle> {
    pub self_key: Option<StoreKey>,
    pub np_collider: NpCollider<N, BH>,
    pub world_key: StoreKey,
    pub shape_key: StoreKey,
}

impl<N: PtReal, BH: NpBodyHandle> Collider<N, BH>{
    pub(crate) fn new(np_collider: NpCollider<N, BH>, world_key: StoreKey, shape_key: StoreKey) -> Self {
        Collider {
            self_key: None,
            np_collider,
            world_key,
            shape_key,
        }
    }
}
