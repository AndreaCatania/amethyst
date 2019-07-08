use crate::storage::StoreTag;

use nphysics3d::object::{
    BodyHandle as NpBodyHandle,
    ColliderHandle as NpColliderHandle,
};

use amethyst_core::ecs::Entity;

use amethyst_phythyst::{
    objects::*,
    servers::BodyMode,
};

pub struct RigidBody {
    pub self_tag: Option<PhysicsBodyTag>,
    pub body_mode: BodyMode,
    pub body_handle: NpBodyHandle,
    pub collider_handle: Option<NpColliderHandle>,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: Option<PhysicsShapeTag>,
    pub entity: Option<Entity>,
}

impl RigidBody {
    pub(crate) fn new(world_tag: PhysicsWorldTag, body_mode: BodyMode) -> Box<Self> {

        Box::new(RigidBody {
            self_tag: None,
            body_mode,
            body_handle: NpBodyHandle::ground(),
            collider_handle: None,
            world_tag,
            shape_tag: None,
            entity: None,
        })
    }
}
