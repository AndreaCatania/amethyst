
use nphysics3d::object::{Body as NpBody, RigidBody as NpRigidBody, BodyHandle as NpBodyHandle, ColliderHandle as NpColliderHandle};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal, objects::*, servers::BodyMode};

use crate::storage::StoreKey;

// TODO rename to Body
pub struct RigidBody<N: PtReal> {
    pub self_key: Option<StoreKey>,
    pub np_body: Box<dyn NpBody<N>>,
    pub body_data: BodyData,
    pub world_key: StoreKey,
    pub collider_key: Option<StoreKey>,
    pub shape_key: Option<StoreKey>,
    pub entity: Option<Entity>,
}

impl<N: PtReal> RigidBody<N> {
    pub(crate) fn new_rigid_body(np_rigid_body: Box<NpRigidBody<N>>, body_mode: BodyMode, world_key: StoreKey) -> Self {
        RigidBody {
            self_key: None,
            np_body: np_rigid_body,
            body_data: BodyData::Rigid(body_mode),
            world_key,
            collider_key: None,
            shape_key: None,
            entity: None,
        }
    }

    pub fn rigid_body(&self) -> Option<&NpRigidBody<N>> {
        self.np_body.downcast_ref::<NpRigidBody<N>>()
    }

    pub fn rigid_body_mut(&self) -> Option<&mut NpRigidBody<N>> {
        self.np_body.downcast_mut::<NpRigidBody<N>>()
    }
}

/// Here are stored extra body information, depending on the body type
pub enum BodyData{
    Rigid(BodyMode),
}
