use nphysics3d::{
    object::BodySet as NpBodySet,
    joint::JointConstraint as NpJointConstraint
};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal};

use crate::storage::StoreKey;

pub struct Joint<N: PtReal, S: NpBodySet<N>> {
    pub self_key: Option<StoreKey>,
    pub np_joint: Box<dyn NpJointConstraint<N, S>>,
    pub world_key: StoreKey,
}

impl<N: PtReal, S: NpBodySet<N>> Joint<N, S>{
    pub(crate) fn new(np_joint: Box<dyn NpJointConstraint<N, S>>, world_key: StoreKey) -> Self {
        Joint {
            self_key: None,
            np_joint,
            world_key,
        }
    }
}