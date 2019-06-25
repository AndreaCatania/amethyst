
use crate::{
    storage::{StoreTag, Storage},
    world::World,
    rigid_body::RigidBody,
};

use amethyst_phythyst::objects::*;

use std::sync::{
    Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use nphysics3d::object::{
    RigidBody as NpRigidBody,
    BodyHandle as NpBodyHandle
};

pub type ServersStorageType = Arc<ServersStorage>;
pub type WorldStorageWrite<'a> = RwLockWriteGuard<'a, Storage<Box<World>>>;
pub type WorldStorageRead<'a> = RwLockReadGuard<'a, Storage< Box<World>>>;
pub type RigidBodyStorageWrite<'a> = RwLockWriteGuard<'a, Storage<Box<RigidBody>>>;
pub type RigidBodyStorageRead<'a> = RwLockReadGuard<'a, Storage<Box<RigidBody>>>;

/// This struct is responsible to hold all the storages
pub struct ServersStorage {
    worlds: Arc<RwLock<Storage<Box<World>>>>,
    rigid_bodies: Arc<RwLock<Storage<Box<RigidBody>>>>,
}

impl ServersStorage {
    pub fn new() -> ServersStorageType {
        Arc::new(ServersStorage {
            worlds: Arc::new(RwLock::new(Storage::new(1, 1))),
            rigid_bodies: Arc::new(RwLock::new(Storage::new(50, 50))),
        })
    }

    pub fn rigid_body<'s>(body_handle: NpBodyHandle, world_tag :StoreTag, storage: &'s WorldStorageRead) -> Option<&'s NpRigidBody<f32>>{
        let world = storage.get(world_tag);
        if let Some(world) = world {
            world.rigid_body(body_handle)
        }else{
            None
        }
    }

}

impl ServersStorage{
    pub fn worlds_w(&self) -> WorldStorageWrite {
        self.worlds.write().unwrap()
    }

    pub fn worlds_r(&self) -> WorldStorageRead {
        self.worlds.read().unwrap()
    }

    pub fn rbodies_w(&self) -> RigidBodyStorageWrite {
        self.rigid_bodies.write().unwrap()
    }

    pub fn rbodies_r(&self) -> RigidBodyStorageRead {
        self.rigid_bodies.read().unwrap()
    }
}