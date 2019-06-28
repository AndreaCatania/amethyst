use crate::{
    rigid_body::RigidBody,
    shape::RigidShape,
    storage::{Storage, StoreTag},
    world::World,
};

use amethyst_phythyst::objects::*;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use nphysics3d::object::{BodyHandle as NpBodyHandle, RigidBody as NpRigidBody};

use nalgebra::RealField;

pub type ServersStorageType<N> = Arc<ServersStorage<N>>;
pub type WorldStorageWrite<'a, N> = RwLockWriteGuard<'a, Storage<Box<World<N>>>>;
pub type WorldStorageRead<'a, N> = RwLockReadGuard<'a, Storage<Box<World<N>>>>;
pub type RigidBodyStorageWrite<'a> = RwLockWriteGuard<'a, Storage<Box<RigidBody>>>;
pub type RigidBodyStorageRead<'a> = RwLockReadGuard<'a, Storage<Box<RigidBody>>>;
pub type ShapeStorageWrite<'a, N> = RwLockWriteGuard<'a, Storage<Box<RigidShape<N>>>>;
pub type ShapeStorageRead<'a, N> = RwLockReadGuard<'a, Storage<Box<RigidShape<N>>>>;

/// This struct is responsible to hold all the storages
pub struct ServersStorage<N: RealField> {
    // TODO is possible to remove RealField here?
    worlds: Arc<RwLock<Storage<Box<World<N>>>>>,
    rigid_bodies: Arc<RwLock<Storage<Box<RigidBody>>>>,
    shapes: Arc<RwLock<Storage<Box<RigidShape<N>>>>>,
}

impl<N: RealField> ServersStorage<N> {
    pub fn new() -> ServersStorageType<N> {
        Arc::new(ServersStorage {
            worlds: Arc::new(RwLock::new(Storage::new(1, 1))),
            rigid_bodies: Arc::new(RwLock::new(Storage::new(50, 50))),
            shapes: Arc::new(RwLock::new(Storage::new(50, 50))),
        })
    }

    pub fn rigid_body<'s>(
        body_handle: NpBodyHandle,
        world_tag: StoreTag,
        storage: &'s WorldStorageRead<N>,
    ) -> Option<&'s NpRigidBody<N>> {
        let world = storage.get(world_tag);
        if let Some(world) = world {
            world.rigid_body(body_handle)
        } else {
            None
        }
    }

    pub fn rigid_body_mut<'s>(
        body_handle: NpBodyHandle,
        world_tag: StoreTag,
        storage: &'s mut WorldStorageWrite<N>,
    ) -> Option<&'s mut NpRigidBody<N>> {
        let mut world = storage.get_mut(world_tag);
        if let Some(world) = world {
            world.rigid_body_mut(body_handle)
        } else {
            None
        }
    }
}

impl<N: RealField> ServersStorage<N> {
    pub fn worlds_w(&self) -> WorldStorageWrite<N> {
        self.worlds.write().unwrap()
    }

    pub fn worlds_r(&self) -> WorldStorageRead<N> {
        self.worlds.read().unwrap()
    }

    pub fn rbodies_w(&self) -> RigidBodyStorageWrite {
        self.rigid_bodies.write().unwrap()
    }

    pub fn rbodies_r(&self) -> RigidBodyStorageRead {
        self.rigid_bodies.read().unwrap()
    }

    pub fn shapes_w(&self) -> ShapeStorageWrite<N> {
        self.shapes.write().unwrap()
    }

    pub fn shapes_r(&self) -> ShapeStorageRead<N> {
        self.shapes.read().unwrap()
    }
}
