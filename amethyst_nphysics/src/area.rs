use amethyst_phythyst::{objects::*, servers::OverlapEvent};
use amethyst_core::ecs::Entity;
use nphysics3d::object::ColliderHandle as NpColliderHandle;

use crate::storage::StoreKey;

pub struct Area {
    pub self_key: Option<StoreKey>,
    pub collider_key: Option<StoreKey>,
    pub world_key: StoreKey,
    pub shape_key: StoreKey,
    pub entity: Option<Entity>,
    pub overlap_events: Vec<OverlapEvent>,
}

impl Area {
    pub(crate) fn new(world_key: StoreKey, shape_key: StoreKey) -> Self {
        Area {
            self_key: None,
            collider_key: None,
            world_key,
            shape_key,
            entity: None,
            overlap_events: Vec::new(),
        }
    }
}
