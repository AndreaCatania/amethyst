use amethyst_phythyst::{
    objects::*,
    servers::{JointDesc, JointPhysicsServerTrait},
    PtReal,
};
use nalgebra::Isometry3;

use crate::{conversors::*, joint::Joint, servers_storage::ServersStorages};

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
        initial_position: &Isometry3<N>,
    ) -> PhysicsHandle<PhysicsJointTag> {
        let mut joints = self.storages.joints_w();
        let key = joints.insert(Box::new(Joint::new(*desc, initial_position.clone())));
        PhysicsHandle::new(store_key_to_joint_tag(key), self.storages.gc.clone())
    }

    fn insert_rigid_body(
        &self,
        joint: PhysicsJointTag,
        body: PhysicsRigidBodyTag,
    ) {

    }

    fn remove_rigid_body(
        &self,
        joint: PhysicsJointTag,
        body: PhysicsRigidBodyTag,
    ) {

    }
}
