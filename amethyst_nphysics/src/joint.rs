use amethyst_phythyst::{servers::JointDesc, PtReal};
use nalgebra::Isometry3;
use nphysics3d::{joint::JointConstraint as NpJointConstraint, object::BodySet as NpBodySet};

use crate::storage::StoreKey;

pub struct Joint<N: PtReal, S: NpBodySet<N>> {
    pub self_key: Option<StoreKey>,
    pub joint_desc: JointDesc,
    pub initial_isometry: Isometry3<N>,
    pub np_joint: Option<Box<dyn NpJointConstraint<N, S>>>,
}

impl<N: PtReal, S: NpBodySet<N>> Joint<N, S> {
    pub(crate) fn new(joint_desc: JointDesc, initial_isometry: Isometry3<N>) -> Self {
        Joint {
            self_key: None,
            joint_desc,
            initial_isometry,
            np_joint: None,
        }
    }
}
