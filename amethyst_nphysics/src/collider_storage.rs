
use ncollide3d::pipeline::object::{
    CollisionObjectSet as NpCollisionObjectSet
};
use nphysics3d::object::{
    Collider as NpCollider,
    ColliderSet,
    BodyHandle as NpBodyHandle,
    ColliderRemovalData as NpColliderRemovalData,
};
use amethyst_phythyst::{
    objects::PhysicsBodyTag,
    PtReal
};

use crate::{
    storage::{
        Storage, StoreKey,
    }
};

pub struct ColliderStorage<N: PtReal, BH: NpBodyHandle>{
    // TODO use Box
    storage: Storage<Box<NpCollider<N, BH>>>,
    /// A list of inserted ID, this list is decremented only when the function `pop_inserted_event` is called
    inserted: Vec<StoreKey>,
    /// A list of removed ID, this list is decremented only when the function `pop_removal_event` is called
    removed: Vec<(StoreKey, NpColliderRemovalData<N, BH>)>,
}

impl<N: PtReal, BH: NpBodyHandle> ColliderStorage<N, BH>{
    pub fn new() -> Self {
        ColliderStorage {
            storage: Storage::new(50, 50),
            inserted: Vec::new(),
            removed: Vec::new(),
        }
    }
}

impl<N: PtReal, BH: NpBodyHandle> Default for ColliderStorage<N, BH> {
    fn default() -> Self {
        ColliderStorage::new()
    }
}

impl<N:PtReal, BH: NpBodyHandle> ColliderStorage<N, BH> {
    pub fn insert_collider(&mut self, collider: Box<NpCollider<N, BH>>) -> StoreKey {
        let key = self.storage.make_opaque(collider);
        self.inserted.push(key);
        key
    }

    pub fn drop_collider(&mut self, key: StoreKey) {
        let res = self.storage.destroy(key);
        if let Some(data) = res {
            if let Some(d) = data.removal_data() {
                self.removed.push((key, d));
            }
        }
    }

    pub fn get_collider(&self, key: StoreKey) -> Option<&Box<NpCollider<N, BH>>> {
        self.storage.get(key)
    }

    pub fn get_collider_mut(&mut self, key: StoreKey) -> Option<&mut Box<NpCollider<N, BH>>> {
        self.storage.get_mut(key)
    }
}

impl<N: PtReal, BH: NpBodyHandle> NpCollisionObjectSet<N> for ColliderStorage<N, BH>{

    type CollisionObject = NpCollider<N, BH>;
    type CollisionObjectHandle = StoreKey;

    fn collision_object(&self, handle: Self::CollisionObjectHandle) -> Option<&Self::CollisionObject>{
        if let Some(collider) = self.storage.get(handle) {
            Some(&collider)
        }else{
            None
        }
    }

    fn foreach(&self, mut f: impl FnMut(Self::CollisionObjectHandle, &Self::CollisionObject)){
        for(i, c) in self.storage.iter() {
            f(i, &c)
        }
    }
}

impl<N: PtReal, BH: NpBodyHandle> ColliderSet<N, BH> for ColliderStorage<N, BH> {

    type Handle = StoreKey;

    fn get(&self, handle: Self::Handle) -> Option<&NpCollider<N, BH>> {
        self.storage.get(handle).map(|c|c.as_ref())
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut NpCollider<N, BH>> {
        self.storage.get_mut(handle).map(|c|c.as_mut())
    }

    fn get_pair_mut(&mut self, handle1: Self::Handle, handle2: Self::Handle) -> (Option<&mut NpCollider<N, BH>>, Option<&mut NpCollider<N, BH>>){
        assert_ne!(handle1, handle2, "Both body handles must not be equal.");
        let b1 = self.get_mut(handle1).map(|b| b as *mut NpCollider<N, BH>);
        let b2 = self.get_mut(handle2).map(|b| b as *mut NpCollider<N, BH>);
        unsafe {
            use std::mem;
            (
                b1.map(|b| mem::transmute(b)),
                b2.map(|b| mem::transmute(b))
            )
        }
    }

    fn contains(&self, handle: Self::Handle) -> bool {
        self.storage.has(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &NpCollider<N, BH>)){
        for(i, c) in self.storage.iter() {
            f(i, c)
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut NpCollider<N, BH>)){
        for(i, c) in self.storage.iter_mut() {
            f(i, c)
        }
    }

    fn pop_insertion_event(&mut self) -> Option<Self::Handle>{
        self.inserted.pop()
    }

    fn pop_removal_event(&mut self) -> Option<(Self::Handle, NpColliderRemovalData<N, BH>)>{
        self.removed.pop()
    }

    fn remove(&mut self, to_remove: Self::Handle) -> Option<&mut NpColliderRemovalData<N, BH>> {
        let collider = self.storage.destroy(to_remove)?;
        if let Some(data) = collider.removal_data() {
            self.removed.push((to_remove, data));
            self.removed.last_mut().map(|r| &mut r.1)
        } else {
            None
        }
    }
}