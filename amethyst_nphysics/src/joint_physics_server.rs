use amethyst_phythyst::{
    objects::*,
    servers::{JointDesc, JointPhysicsServerTrait},
    PtReal,
};
use log::error;
use nalgebra::Isometry3;
use nphysics3d::{
    joint::{FixedConstraint as NpFixedConstraint},
    object::{
        BodyPartHandle as NpBodyPartHandle,
        BodySet as NpBodySet,Body
    }
};

use crate::{
    conversors::*,
    joint::Joint,
    servers_storage::{JointsStorageWrite, BodiesStorageRead, ServersStorages},
    storage::StoreKey,
};

pub struct JointNpServer<N: PtReal> {
    storages: ServersStorages<N>,
}

impl<N: PtReal> JointNpServer<N> {
    pub fn new(storages: ServersStorages<N>) -> Self {
        Self { storages }
    }

    pub fn update_internal_joint(bodies: &mut BodiesStorageRead<N>, joints: &mut JointsStorageWrite<N>, joint_key: StoreKey) {
        if let Some(joint) = joints.get_joint_mut(joint_key) {
            if joint.np_joint.is_some() {
                if joint.body_0.is_none() || joint.body_1.is_none() {
                    // Remove joint
                }
            } else{
                if joint.body_0.is_some() && joint.body_1.is_some() {
                    // Create the joint

                    let body_0 = bodies
                        .get_body(joint.body_0.unwrap().0)
                        .map(|v|v.np_body.);
                    //let body_1 = bodies.get_body(joint.body_1.unwrap().0);

                    match joint.joint_desc {
                        JointDesc::Fixed => {


                            /*NpFixedConstraint::new(
                                joint.body_0.map(|v|NpBodyPartHandle(v.0, v.1)).unwrap(),
                            joint.body_0.map(|v|NpBodyPartHandle(v.0, v.1)).unwrap(),

                            )*/
                        }
                    }
                }
            }
        }
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

    fn insert_rigid_body(&self, joint_tag: PhysicsJointTag, body: PhysicsRigidBodyTag) {
        let joint_key = joint_tag_to_store_key(joint_tag);
        let mut joints = self.storages.joints_w();

        if let Some(joint) = joints.get_joint_mut(joint_key) {
            if joint.body_0.is_none() {
                joint.body_0 = Some((rigid_tag_to_store_key(body), 0));
            } else if joint.body_1.is_none() {
                joint.body_1 = Some((rigid_tag_to_store_key(body), 0));
            } else {
                error!("This joint is already joining two other bodies, and you can't add more. Remove one of them if you want to constraint this new joint.");
                return;
            }
        } else {
            error!("Joint tag not found!");
        }

        Self::update_internal_joint(&mut self.storages.bodies_r(), &mut joints, joint_key);
    }

    fn remove_rigid_body(&self, joint: PhysicsJointTag, body: PhysicsRigidBodyTag) {}
}
