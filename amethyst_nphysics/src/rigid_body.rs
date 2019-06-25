
use ncollide3d::{
    shape::{
        ShapeHandle as NcShapeHandle,
        Ball as NcBall,
    },
};

use nphysics3d::{
    object::{
        RigidBody as NpRigidBody,
        RigidBodyDesc as NpRigidBodyDesc,
        ColliderDesc as NpColliderDesc,
        BodyHandle as NpBodyHandle,
        BodyStatus as NpBodyStatus,
    },
};

use nalgebra::{
    RealField,
    Isometry3,
    Vector3,
};

use crate::{
    world::World,
};

use amethyst_phythyst::objects::*;

pub struct RigidBody {
    pub body_handle :NpBodyHandle,
    pub world_tag :PhysicsWorldTag,
}

impl RigidBody {
    pub fn new(body_handle :NpBodyHandle, world_tag :PhysicsWorldTag) -> Box<Self> {

        Box::new(RigidBody {
            body_handle,
            world_tag,
        })
    }
}