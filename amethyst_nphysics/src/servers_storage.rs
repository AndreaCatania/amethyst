use crate::{
    area::Area,
    rigid_body::RigidBody,
    shape::RigidShape,
    storage::{Storage, StoreTag},
    world::World,
    PtReal,
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
pub type AreaStorageWrite<'a> = RwLockWriteGuard<'a, Storage<Box<Area>>>;
pub type AreaStorageRead<'a> = RwLockReadGuard<'a, Storage<Box<Area>>>;
pub type ShapeStorageWrite<'a, N> = RwLockWriteGuard<'a, Storage<Box<RigidShape<N>>>>;
pub type ShapeStorageRead<'a, N> = RwLockReadGuard<'a, Storage<Box<RigidShape<N>>>>;

/// This struct is responsible to hold all the storages
///
/// ## Multi threading issue
/// This a simplified version of the actual way to handle the storages:
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=use%20std%3A%3Async%3A%3A%7B%0A%20%20%20%20RwLock%2C%0A%20%20%20%20Arc%2C%0A%7D%3B%0A%0Astruct%20WorldStorage%7B%0A%20%20%20%20%20pub%20worlds%3A%20Vec%3CArc%3CRwLock%3CWorld%3E%3E%3E%2C%20%20%20%0A%7D%0A%0Astruct%20World%7B%0A%20%20%20%20pub%20bodies%3A%20Vec%3CBox%3CBody%3E%3E%2C%0A%7D%0A%0A%23%5Bderive(Debug)%5D%0Astruct%20Body%7B%0A%20%20%20%20pub%20i%3A%20i32%2C%0A%20%20%20%20pub%20weight%3A%20f32%2C%0A%7D%0A%0Afn%20main()%7B%0A%0A%20%20%20%20%2F%2F%20Create%20Storage%2C%20World%20and%203%20bodies%20owned%20by%20the%20world%0A%20%20%20%20let%20mut%20storage%20%3D%20WorldStorage%7B%0A%20%20%20%20%20%20%20%20worlds%3A%20vec!()%2C%0A%20%20%20%20%7D%3B%0A%20%20%20%20%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20%2F%2F%20These%20are%203%20independent%20bodies%0A%20%20%20%20%20%20%20%20let%20body_1%20%3D%20Box%3A%3Anew(Body%7Bi%3A%201%2C%20weight%3A%2010.0%7D)%3B%0A%20%20%20%20%20%20%20%20let%20body_2%20%3D%20Box%3A%3Anew(Body%7Bi%3A%202%2C%20weight%3A%2010.0%7D)%3B%0A%20%20%20%20%20%20%20%20let%20body_3%20%3D%20Box%3A%3Anew(Body%7Bi%3A%203%2C%20weight%3A%2010.0%7D)%3B%0A%20%20%20%20%20%20%20%20%0A%20%20%20%20%20%20%20%20storage.worlds.push(Arc%3A%3Anew(RwLock%3A%3Anew(World%7Bbodies%3A%20vec!(body_1%2C%20body_2%2C%20body_3)%2C%7D)))%3B%0A%20%20%20%20%7D%0A%20%20%20%20%0A%20%20%20%20mutate_parallel(%26storage%2C%200)%3B%0A%20%20%20%20mutate_parallel(%26storage%2C%201)%3B%0A%20%20%20%20mutate_parallel(%26storage%2C%202)%3B%0A%20%20%20%20%0A%20%20%20%20%0A%20%20%20%20let%20world%20%3D%20storage.worlds%5B0%5D.read().unwrap()%3B%0A%20%20%20%20for%20b%20in%20%26world.bodies%20%7B%0A%20%20%20%20%20%20%20%20dbg!(b)%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20mutate_parallel(storage%3A%20%26WorldStorage%2C%20body_id%3A%20usize)%7B%0A%20%20%20%20let%20mut%20world%20%3D%20storage.worlds%5B0%5D.write().unwrap()%3B%20%20%20%0A%20%20%20%20%0A%20%20%20%20world.bodies%5Bbody_id%5D.weight%20%3D%2044.0%3B%0A%7D%0A
/// The world internal storage is not thread safe, this mean that is mandatory have a mutable World
/// in order to retrieve a **Mutable** body.
///
/// The problem is that taking a mutable World using the `RwLock::write()` function make all others
/// threads to wait the unlock of the world.
///
/// Since each call like apply_force, or set_velocity, or set_friction need a mutable body that
/// can be taken only if the world is mutable.
/// Again to take the World mutable I have to use `RwLock::write()` that synchronize the execution.
///
/// A solution to this problem would be support add multithreading support on NPhysics
pub struct ServersStorage<N: PtReal> {
    // TODO is possible to remove RealField here?
    pub(crate) gc: Arc<RwLock<PhysicsGarbageCollector>>,
    worlds: Arc<RwLock<Storage<Box<World<N>>>>>,
    rigid_bodies: Arc<RwLock<Storage<Box<RigidBody>>>>,
    areas: Arc<RwLock<Storage<Box<Area>>>>,
    shapes: Arc<RwLock<Storage<Box<RigidShape<N>>>>>,
}

impl<N: PtReal> ServersStorage<N> {
    pub fn new() -> ServersStorageType<N> {
        Arc::new(ServersStorage {
            gc: Arc::new(RwLock::new(PhysicsGarbageCollector::default())),
            worlds: Arc::new(RwLock::new(Storage::new(1, 1))),
            rigid_bodies: Arc::new(RwLock::new(Storage::new(50, 50))),
            areas: Arc::new(RwLock::new(Storage::new(50, 50))),
            shapes: Arc::new(RwLock::new(Storage::new(50, 50))),
        })
    }

    // TODO why use this?
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

    // TODO why use this?
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

impl<N: PtReal> ServersStorage<N> {
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

    pub fn areas_w(&self) -> AreaStorageWrite {
        self.areas.write().unwrap()
    }

    pub fn areas_r(&self) -> AreaStorageRead {
        self.areas.read().unwrap()
    }

    pub fn shapes_w(&self) -> ShapeStorageWrite<N> {
        self.shapes.write().unwrap()
    }

    pub fn shapes_r(&self) -> ShapeStorageRead<N> {
        self.shapes.read().unwrap()
    }
}
