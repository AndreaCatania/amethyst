use ncollide3d::{
    shape::{
        ShapeHandle
    },
};
use nphysics3d::{
    object::{
        RigidBody,
        RigidBodyDesc,
        ColliderDesc,
    },
};

pub(crate) struct ARigidBody{

}

impl ARigidBody{
    pub fn new() -> Box<Self> {

        //let shape = ShapeHandle::new(Ball::new(1.5));
//        let mut collider_desc = Box::new(ColliderDesc::new(shape));
//        collider_desc.set_density(1.0);
//        collider_desc.set_translation(Vector3::y() * 5.0);
//
//        let a = RigidBodyDesc::new()
//            .collider(&collider_desc);

        Box::new(ARigidBody{})
    }
}