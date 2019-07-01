
use amethyst_phythyst::objects::*;

use nphysics3d::object::{
    ColliderHandle as NpColliderHandle,
};

pub struct Area{
    pub collider_handle: Option<NpColliderHandle>,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: PhysicsShapeTag,
}

impl Area{
    pub fn new(collider_handle: Option<NpColliderHandle>, world_tag: PhysicsWorldTag, shape_tag: PhysicsShapeTag) -> Self{
        Area{
            collider_handle,
            world_tag,
            shape_tag,
        }
    }
}