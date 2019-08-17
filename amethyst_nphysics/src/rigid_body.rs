
use nphysics3d::object::{Body as NpBody, RigidBody as NpRigidBody, BodyHandle as NpBodyHandle, ColliderHandle as NpColliderHandle};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal, objects::*, servers::{BodyMode, OverlapEvent}};

use crate::storage::StoreKey;

// TODO rename to Body
/// Store information about a body
///
/// A body is:
/// - Rigid Body (Disabled, Dynamic, Static, Kinematic)
/// - Area
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
    pub(crate) fn new_rigid_body(np_rigid_body: Box<NpRigidBody<N>>, world_key: StoreKey) -> Self {
        RigidBody {
            self_key: None,
            np_body: np_rigid_body,
            body_data: BodyData::Rigid,
            world_key,
            collider_key: None,
            shape_key: None,
            entity: None,
        }
    }

    pub(crate) fn new_area(np_rigid_body: Box<NpRigidBody<N>>, world_key: StoreKey) -> Self {
        RigidBody {
            self_key: None,
            np_body: np_rigid_body,
            body_data: BodyData::Area(Vec::new()),
            world_key,
            collider_key: None,
            shape_key: None,
            entity: None,
        }
    }

    pub fn rigid_body(&self) -> Option<&NpRigidBody<N>> {
        self.np_body.downcast_ref::<NpRigidBody<N>>()
    }

    pub fn rigid_body_mut(&mut self) -> Option<&mut NpRigidBody<N>> {
        self.np_body.downcast_mut::<NpRigidBody<N>>()
    }
}

/// Here are stored extra body information, depending on the body type
pub enum BodyData{
    Rigid,
    Area(Vec<OverlapEvent>),
}
