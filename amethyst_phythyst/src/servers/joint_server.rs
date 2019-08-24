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
        initial_position: Isometry3<N>,
    ) -> PhysicsHandle<PhysicsJointTag>;

    /// Sets the rigid body handles, and creates the actual joint.
    /// Can't be called twice on the same `PhysicsJointTag`.
    ///
    /// This function is called automatically when a `PhysicsHandle<PhysicsJointTag>` is assigned to
    /// two `Entities` that both have a `PhysicsHandle<PhysicsRigidBodyTag>`.
    ///
    /// So, you have to just create the joint using the function `create_joint`.
    /// To drop a joint, all the handles, must be dropped.
    fn init_with_rigid_bodies(
        &self,
        joint: PhysicsJointTag,
        body_0: PhysicsRigidBodyTag,
        body_1: PhysicsRigidBodyTag,
    );
}

/// Joint description, used during the joint creation.
#[derive(Clone, Debug)]
pub enum JointDesc {
    Fixed,
}
