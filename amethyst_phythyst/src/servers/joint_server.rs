use amethyst_core::components::Transform;
use nalgebra::{Isometry3, Point3, RealField, Vector3};

use crate::objects::*;

/// Trait that defines the *Joint* server capabilities.
pub trait JointPhysicsServerTrait<N: crate::PtReal> {
    /// Creates a new joint.
    ///
    /// The parameter `initial_position` is used to calculates the body offset to the joint.
    ///
    /// The joint created by this function is not yet active; Indeed, you have to assign the
    /// `PhysicsHandle<PhysicsJointTag>` returned, to the two `Entities` that you want to constraint.
    ///
    /// To remove this joint, is necessary to drop all its handles.
    fn create_joint(
        &self,
        desc: &JointDesc,
        initial_position: &Isometry3<N>,
    ) -> PhysicsHandle<PhysicsJointTag>;

    /// Insert the rigid body to the joint, and in case creates the actual joint.
    /// It doesn't accept more than two handles per time.
    ///
    /// This function is called automatically when a `PhysicsHandle<PhysicsJointTag>` is assigned to
    /// an `Entity` that has a `PhysicsHandle<PhysicsRigidBodyTag>`.
    ///
    /// So, you have to just create the joint using the function `create_joint`.
    fn insert_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag);

    /// Remove the rigid body to the joint.
    ///
    /// This function is called automatically when a `PhysicsHandle<PhysicsJointTag>` is removed from
    /// an `Entity`.
    ///
    /// To drop a joint, you simply need to drop the handle.
    fn remove_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag);
}

/// Joint description, used during the joint creation.
#[derive(Copy, Clone, Debug)]
pub enum JointDesc {
    Fixed,
}
