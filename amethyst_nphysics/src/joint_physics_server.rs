use amethyst_phythyst::{
    objects::*,
    servers::{JointDesc, JointPhysicsServerTrait},
    PtReal,
};
use nalgebra::Isometry3;

use crate::servers_storage::ServersStorages;

pub struct JointNpServer<N: PtReal> {
    storages: ServersStorages<N>,
}

impl<N: PtReal> JointNpServer<N> {
    pub fn new(storages: ServersStorages<N>) -> Self {
        Self { storages }
    }
}

impl<N: PtReal> JointPhysicsServerTrait<N> for JointNpServer<N> {
    fn create_joint(
        &self,
        desc: &JointDesc,
        initial_position: Isometry3<N>,
    ) -> PhysicsHandle<PhysicsJointTag> {
        unimplemented!();
    }

    fn init_with_rigid_bodies(
        &self,
        joint: PhysicsJointTag,
        body_0: PhysicsRigidBodyTag,
        body_1: PhysicsRigidBodyTag,
    ) {

    }
}
