use amethyst_core::ecs::Entity;
use amethyst_phythyst::{
    objects::*,
    servers::{BodyMode, OverlapEvent},
    PtReal,
};
use nphysics3d::object::{
    Body as NpBody, BodyHandle as NpBodyHandle, ColliderHandle as NpColliderHandle,
    RigidBody as NpRigidBody,
};

use crate::storage::StoreKey;

/// Store information about a body
///
/// A body is:
/// - Rigid - RigidBody(Disabled, Dynamic, Static, Kinematic)
/// - Area - RigidBody(Static)
pub struct Body<N: PtReal> {
    pub self_key: Option<StoreKey>,
    pub np_body: Box<dyn NpBody<N>>,
    pub body_data: BodyData,
    pub world_key: StoreKey,
    pub collider_key: Option<StoreKey>,
    pub shape_key: Option<StoreKey>,
    pub entity: Option<Entity>,
}

impl<N: PtReal> Body<N> {
    /// Creates a Rigid Body `Body`
    pub(crate) fn new_rigid_body(np_rigid_body: Box<NpRigidBody<N>>, world_key: StoreKey) -> Self {
        Body {
            self_key: None,
            np_body: np_rigid_body,
            body_data: BodyData::Rigid,
            world_key,
            collider_key: None,
            shape_key: None,
            entity: None,
        }
    }

    /// Creates an Area `Body`
    pub(crate) fn new_area(np_rigid_body: Box<NpRigidBody<N>>, world_key: StoreKey) -> Self {
        Body {
            self_key: None,
            np_body: np_rigid_body,
            body_data: BodyData::Area(Vec::new()),
            world_key,
            collider_key: None,
            shape_key: None,
            entity: None,
        }
    }

    /// Returns some with a rigid body reference if this body is a RigidBody.
    ///
    /// Note that the area is a RigidBody.
    pub fn rigid_body(&self) -> Option<&NpRigidBody<N>> {
        self.np_body.downcast_ref::<NpRigidBody<N>>()
    }

    /// Returns some with a rigid body mut reference if this body is a RigidBody.
    ///
    /// Note that the area is a RigidBody.
    pub fn rigid_body_mut(&mut self) -> Option<&mut NpRigidBody<N>> {
        self.np_body.downcast_mut::<NpRigidBody<N>>()
    }
}

/// Here are stored extra body information, depending on the body type
#[derive(Debug, PartialEq)]
pub enum BodyData {
    Rigid,
    Area(Vec<OverlapEvent>),
}
