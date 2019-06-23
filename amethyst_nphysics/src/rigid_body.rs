use ncollide3d::{
    shape::{
        ShapeHandle
    },
};
use nphysics3d::{
    object::{
        RigidBody as NpRigidBody,
        RigidBodyDesc as NpRigidBodyDesc,
        ColliderDesc as NpColliderDesc,
    },
};

pub struct RigidBody {
    pub a: i32,
}

impl RigidBody {
    pub fn new() -> Box<Self> {

        //let shape = ShapeHandle::new(Ball::new(1.5));
//        let mut collider_desc = Box::new(ColliderDesc::new(shape));
//        collider_desc.set_density(1.0);
//        collider_desc.set_translation(Vector3::y() * 5.0);
//
//        let a = NpRigidBodyDesc::new();
//            .collider(&collider_desc);

        Box::new(RigidBody {
            a: 10,
        })
    }
}