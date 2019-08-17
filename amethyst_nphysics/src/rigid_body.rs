
use nphysics3d::object::{Body as NpBody, BodyHandle as NpBodyHandle, ColliderHandle as NpColliderHandle};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{PtReal, objects::*, servers::BodyMode};

use crate::storage::StoreKey;

// TODO rename to Body
// The idea is to store the per type data using a pointer or something like that.
pub struct RigidBody<N: PtReal> {
    pub self_tag: Option<StoreKey>,
    pub np_body: Box<dyn NpBody<N>>,
    pub body_mode: BodyMode,
    //pub body_handle: NpBodyHandle,
    //pub collider_handle: Option<NpColliderHandle>,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: Option<PhysicsShapeTag>,
    pub entity: Option<Entity>,
}

impl<N: PtReal> RigidBody<N> {
    pub(crate) fn new(np_body: Box<dyn NpBody<N>>, world_tag: PhysicsWorldTag, body_mode: BodyMode) -> Box<Self> {
        Box::new(RigidBody {
            self_tag: None,
            np_body,
            body_mode,
            //body_handle: NpBodyHandle::ground(),
            //collider_handle: None,
            world_tag,
            shape_tag: None,
            entity: None,
        })
    }
}
