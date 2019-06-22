
use crate::{
    storage::Storage,
    rigid_body::ARigidBody,
};
use nphysics3d::world::World;
use std::sync::{
    Arc, RwLock, RwLockWriteGuard, RwLockReadGuard
};
use core::borrow::{Borrow, BorrowMut};

/// This struct is responsible to hold all the storages
pub struct Storages{
    pub worlds: Storage<Box<World<f32>>>,
    pub rigid_bodies: Storage<Box<ARigidBody>>,
}

impl Storages{
    pub fn new() -> Arc<RwLock<Self>>{
        Arc::new(RwLock::new(Storages{
            worlds: Storage::new(1, 1),
            rigid_bodies: Storage::new(50, 50),
        }))
    }
}

#[macro_export]
macro_rules! storage_read{
    ($x:expr) => {
        $x.read().unwrap()
    }
}

#[macro_export]
macro_rules! storage_write{
    ($x:expr) => {
        $x.write().unwrap()
    }
}