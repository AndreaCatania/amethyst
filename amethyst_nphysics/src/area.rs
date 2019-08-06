use amethyst_phythyst::{objects::*, servers::OverlapEvent};

use amethyst_core::ecs::Entity;

use nphysics3d::object::ColliderHandle as NpColliderHandle;

pub struct Area {
    pub self_tag: Option<PhysicsAreaTag>,
    pub collider_handle: Option<NpColliderHandle>,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: PhysicsShapeTag,
    pub entity: Option<Entity>,
    pub overlap_events: Vec<OverlapEvent>,
}

impl Area {
    pub(crate) fn new(world_tag: PhysicsWorldTag, shape_tag: PhysicsShapeTag) -> Self {
        Area {
            self_tag: None,
            collider_handle: None,
            world_tag,
            shape_tag,
            entity: None,
            overlap_events: Vec::new(),
        }
    }
}
