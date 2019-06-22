
use crate::{
    storage::Storage,
    rigid_body::ARigidBody,
};
use nphysics3d::world::World;
use std::sync::{
    Arc, RwLock,
};

pub type ServersStorageType = Arc<RwLock<ServersStorage>>;

/// This struct is responsible to hold all the storages
pub struct ServersStorage {
    pub worlds: Storage<Box<World<f32>>>,
    pub rigid_bodies: Storage<Box<ARigidBody>>,
}

impl ServersStorage {
    pub fn new() -> ServersStorageType {
        Arc::new(RwLock::new(ServersStorage {
            worlds: Storage::new(1, 1),
            rigid_bodies: Storage::new(50, 50),
        }))
    }
}

#[macro_export]
macro_rules! storage_read{
    ($x:ident) => {
        $x.storages.read().unwrap()
    };
    ($x:expr) => {
        $x.read().unwrap()
    }
}

#[macro_export]
macro_rules! storage_write{
    ($x:ident) => {
        $x.storages.write().unwrap()
    };
    ($x:expr) => {
        $x.write().unwrap()
    }
}