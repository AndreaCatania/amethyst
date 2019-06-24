
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

struct RigidBodyData<N: RealField>{
    transformation: Isometry3<N>,
}

impl<N: RealField> Default for RigidBodyData<N>{
    fn default() -> Self {
        RigidBodyData{
            transformation: Isometry3::new(nalgebra::zero(), nalgebra::zero()),
        }
    }
}

// TODO in theory I'll be able to remove this if NpPhysics will give the possibility to handle
// the pointer directly to the user.
// https://github.com/rustsim/nphysics/issues/205
pub enum RBPhase<N: RealField>{
    OutWorld(Box<RigidBodyData<N>>),
    InWorld(NpBodyHandle, PhysicsWorldTag)
}

impl<N: RealField> RBPhase<N> {
    pub fn is_in_world(&self) -> bool {
        match self {
            RBPhase::InWorld(..) => true,
            _ => false,
        }
    }
}

impl<N: RealField> PartialEq for RBPhase<N> {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct RigidBody {
    pub phase: RBPhase<f32>,
}

impl RigidBody {
    pub fn new() -> Box<Self> {

        Box::new(RigidBody {
            phase: RBPhase::OutWorld(Box::new(RigidBodyData::default()))
        })
    }

    pub fn set_world(&mut self, world_tag: PhysicsWorldTag, world: &mut World){

        fail_cond!(self.phase.is_in_world());

        let mut collider_desc = NpColliderDesc::new(NcShapeHandle::new(NcBall::new(0.01)) )
            .density(0.01);
        let rb = NpRigidBodyDesc::new()
            .collider(&collider_desc)
            .set_translation(Vector3::new(5.0, 5.0, 5.0))
            .build(world);

        self.phase = RBPhase::InWorld(rb.handle(), world_tag);
    }

    // This function is here mainly because I would like to handle all the possible things inside
    // the server rather creating a function per each thing.
    pub fn get_handle(&self) -> Option<(NpBodyHandle, PhysicsWorldTag)> {
        if let RBPhase::InWorld(phase, world_tag) = self.phase {
            Some((phase, world_tag ))
        } else {
            None
        }
    }

}