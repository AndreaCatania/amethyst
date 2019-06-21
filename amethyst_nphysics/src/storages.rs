
use crate::{
    storage::Storage,
    rigid_body::ARigidBody,
};
use nphysics3d::world::World;
use std::{
    rc::Rc,
    cell::RefCell,
};

/// This struct is responsible to hold all the storages
pub(crate) struct Storages{
    pub worlds: Storage<Box<World<f32>>>,
    pub rigid_bodies: Storage<Box<ARigidBody>>,
}

impl Storages{
    pub fn new() -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Storages{
            worlds: Storage::new(1, 1),
            rigid_bodies: Storage::new(50, 50),
        }))
    }
}