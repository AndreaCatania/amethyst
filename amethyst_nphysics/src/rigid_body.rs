use ncollide3d::shape::{Ball as NcBall, ShapeHandle as NcShapeHandle};

use nphysics3d::object::{
    BodyHandle as NpBodyHandle, BodyStatus as NpBodyStatus, ColliderDesc as NpColliderDesc,
    ColliderHandle as NpColliderHandle, RigidBody as NpRigidBody, RigidBodyDesc as NpRigidBodyDesc,
};

use nalgebra::{Isometry3, RealField, Vector3};

use crate::world::World;

use amethyst_phythyst::objects::*;

pub struct RigidBody {
    pub body_handle: NpBodyHandle,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: Option<PhysicsShapeTag>,
    pub collider_handle: Option<NpColliderHandle>,
}

impl RigidBody {
    pub fn new(body_handle: NpBodyHandle, world_tag: PhysicsWorldTag) -> Box<Self> {
        Box::new(RigidBody {
            body_handle,
            world_tag,
            shape_tag: None,
            collider_handle: None,
        })
    }
}
