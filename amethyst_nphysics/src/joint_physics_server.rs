use amethyst_phythyst::{
    objects::*,
    servers::{JointDesc, JointPhysicsServerTrait},
    PtReal,
};
use log::error;
use nalgebra::Isometry3;
use nphysics3d::{
    joint::FixedConstraint as NpFixedConstraint,
    object::{Body, BodyPartHandle as NpBodyPartHandle, BodySet as NpBodySet},
};

use crate::{
    conversors::*,
    joint::Joint,
    servers_storage::{BodiesStorageRead, BodiesStorageWrite, JointsStorageWrite, ServersStorages},
    storage::StoreKey,
    RBodyNpServer,
};

pub struct JointNpServer<N: PtReal> {
    storages: ServersStorages<N>,
}

impl<N: PtReal> JointNpServer<N> {
    pub fn new(storages: ServersStorages<N>) -> Self {
        Self { storages }
    }
}

// This is a collection of function that can be used by other servers to perform some common
// operations on the joints.
impl<N: PtReal> JointNpServer<N> {
    pub fn drop_joint(
        joint_tag: PhysicsJointTag,
        joints: &mut JointsStorageWrite<N>,
        bodies: &mut BodiesStorageWrite<N>,
    ) {
        let j_key = joint_tag_to_store_key(joint_tag);

        // Active the constraint bodies
        if let Some(joint) = joints.get_joint(j_key) {
            if joint.body_0.is_some() {
                RBodyNpServer::active_body(joint.body_0.unwrap().0, bodies);
            }
            if joint.body_1.is_some() {
                RBodyNpServer::active_body(joint.body_1.unwrap().0, bodies);
            }
        }

        joints.drop_joint(j_key);
    }

    pub fn update_internal_joint(
        joint_key: StoreKey,
        joints: &mut JointsStorageWrite<N>,
        bodies: &mut BodiesStorageRead<N>,
    ) {
        if let Some(joint) = joints.get_joint_mut(joint_key) {
            if joint.np_joint.is_some() {
                if joint.body_0.is_none() || joint.body_1.is_none() {
                    // -- Remove joint --

                    joint.np_joint = None;
                    joints.notify_joint_removed(joint_key);
                }
            } else {
                if joint.body_0.is_some() && joint.body_1.is_some() {
                    // -- Create the joint --

                    let body_0_trsf = bodies
                        .get_body(joint.body_0.unwrap().0)
                        .map(|v| v.body_transform());
                    let body_1_trsf = bodies
                        .get_body(joint.body_1.unwrap().0)
                        .map(|v| v.body_transform());
                    fail_cond!(body_0_trsf.is_none() || body_1_trsf.is_none());

                    let anchor_0: Isometry3<N> =
                        body_0_trsf.unwrap().inverse() * &joint.initial_isometry;
                    let anchor_1: Isometry3<N> =
                        body_1_trsf.unwrap().inverse() * &joint.initial_isometry;

                    match joint.joint_desc {
                        JointDesc::Fixed => {
                            let np_joint = NpFixedConstraint::new(
                                joint.body_0.map(|v| NpBodyPartHandle(v.0, v.1)).unwrap(),
                                joint.body_1.map(|v| NpBodyPartHandle(v.0, v.1)).unwrap(),
                                anchor_0.translation.vector.into(),
                                anchor_0.rotation,
                                anchor_1.translation.vector.into(),
                                anchor_1.rotation,
                            );
                            joint.np_joint = Some(Box::new(np_joint));
                        }
                    }
                    joints.notify_joint_created(joint_key);
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
        joints.get_joint_mut(key).unwrap().self_key = Some(key);
        PhysicsHandle::new(store_key_to_joint_tag(key), self.storages.gc.clone())
    }

    fn insert_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag) {
        let joint_key = joint_tag_to_store_key(joint_tag);
        let mut joints = self.storages.joints_w();

        if let Some(joint) = joints.get_joint_mut(joint_key) {
            if joint.body_0.is_none() {
                joint.body_0 = Some((rigid_tag_to_store_key(body_tag), 0));
            } else if joint.body_1.is_none() {
                joint.body_1 = Some((rigid_tag_to_store_key(body_tag), 0));
            } else {
                error!("This joint is already joining two other bodies, and you can't add more. Remove one of them if you want to constraint this new joint.");
                return;
            }
        } else {
            error!("Joint tag not found!");
        }

        Self::update_internal_joint(joint_key, &mut joints, &mut self.storages.bodies_r());
    }

    fn remove_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag) {
        let joint_key = joint_tag_to_store_key(joint_tag);
        let mut joints = self.storages.joints_w();

        {
            let mut bodies = self.storages.bodies_w();

            if let Some(joint) = joints.get_joint_mut(joint_key) {
                if joint.body_0.is_some() {
                    RBodyNpServer::active_body(joint.body_0.unwrap().0, &mut bodies);
                }
                if joint.body_1.is_some() {
                    RBodyNpServer::active_body(joint.body_1.unwrap().0, &mut bodies);
                }

                if let Some(true) = joint
                    .body_0
                    .map(|v| v.0 == rigid_tag_to_store_key(body_tag))
                {
                    joint.body_0 = None;
                } else if let Some(true) = joint
                    .body_1
                    .map(|v| v.0 == rigid_tag_to_store_key(body_tag))
                {
                    joint.body_1 = None;
                } else {
                    error!("The body was not found in this joint");
                }
            } else {
                // Nothing, the joint could be already removed
            }
        }

        Self::update_internal_joint(joint_key, &mut joints, &mut self.storages.bodies_r());
    }
}