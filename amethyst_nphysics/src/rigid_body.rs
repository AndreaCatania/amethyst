
use nphysics3d::object::{Body as NpBody, BodyHandle as NpBodyHandle, ColliderHandle as NpColliderHandle};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal, objects::*, servers::BodyMode};

use crate::storage::StoreKey;

// TODO rename to Body
// The idea is to store the per type data using a pointer or something like that.
pub struct RigidBody<N: PtReal> {
    pub self_key: Option<StoreKey>,
    pub np_body: Box<dyn NpBody<N>>,
    pub body_mode: BodyMode,
    pub collider_key: Option<StoreKey>,
    pub world_key: StoreKey,
    pub shape_key: Option<StoreKey>,
    pub entity: Option<Entity>,
}

impl<N: PtReal> RigidBody<N> {
    pub(crate) fn new(np_body: Box<dyn NpBody<N>>, world_key: StoreKey, body_mode: BodyMode) -> Box<Self> {
        Box::new(RigidBody {
            self_key: None,
            np_body,
            body_mode,
            collider_key: None,
            world_key,
            shape_key: None,
            entity: None,
        })
    }
}
