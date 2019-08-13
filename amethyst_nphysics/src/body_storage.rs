
use nphysics3d::object::{
    Body,
    BodySet,
};
use amethyst_phythyst::{
    objects::PhysicsBodyTag,
    PtReal
};

use crate::{
    rigid_body::RigidBody,
    storage::{
        Storage,StoreKey,
    }
};

fn body_tag_to_store_key(tag: PhysicsBodyTag) -> StoreKey {
    StoreKey::new(0,0)
}

fn store_key_to_body_tag(key: StoreKey) -> PhysicsBodyTag {
    key.map(|k| PhysicsBodyTag(std::num::NonZeroU64::new(1).unwrap()))
}

pub(crate) struct BodyStorage<N: PtReal>{
    storage: Storage<RigidBody<N>>,
    /// A list of removed ID, this list is decremented only when the function `pop_removal_event` is called
    removed: Vec<StoreKey>,
}

impl<N: PtReal> BodyStorage<N>{
    fn new() -> Self {
        BodyStorage {
            storage: Storage::new(50, 50),
            removed: Vec::new(),
        }
    }
}

impl<N: PtReal> Default for BodyStorage<N> {
    fn default() -> Self {
        BodyStorage::new()
    }
}

impl<N:PtReal> BodyStorage<N> {
    pub fn insert(&mut self, body: RigidBody<N>) -> StoreKey {
        self.storage.make_opaque(body)
    }

    pub fn drop(&mut self, key: StoreKey) {
        self.storage.destroy(key);
        self.removed.push(key);
    }

    pub fn get_body(&self, key: StoreKey) -> Option<&RigidBody<N>> {
        self.storage.get(key)
    }

    pub fn get_body_mut(&mut self, key: StoreKey) -> Option<&mut RigidBody<N>> {
        self.storage.get_mut(key)
    }
}

impl<N: PtReal> BodySet<N> for BodyStorage<N> {

    type Body = dyn Body<N>;
    type Handle = StoreKey;

    fn get(&self, handle: Self::Handle) -> Option<&Self::Body>{
        if let Some(body) = self.storage.get(handle) {
            Some(body.np_body.as_ref())
        }else{
            None
        }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::Body>{
        if let Some(body) = self.storage.get_mut(handle) {
            Some(body.np_body.as_mut())
        }else{
            None
        }
    }

    fn get_pair_mut(&mut self, handle1: Self::Handle, handle2: Self::Handle) -> (Option<&mut Self::Body>, Option<&mut Self::Body>){
        assert_ne!(handle1, handle2, "Both body handles must not be equal.");
        let b1 = self.get_mut(handle1).map(|b| b as *mut dyn Body<N>);
        let b2 = self.get_mut(handle2).map(|b| b as *mut dyn Body<N>);
        unsafe {
            use std::mem;
            (
                b1.map(|b| mem::transmute(b)),
                b2.map(|b| mem::transmute(b))
            )
        }
    }

    fn contains(&self, handle: Self::Handle) -> bool{
        self.storage.has(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &Self::Body)){
        for(h, b) in self.storage.iter() {
            f(h, b.np_body.as_ref())
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut Self::Body)){
        for(h, b) in self.storage.iter_mut() {
            f(h, b.np_body.as_mut())
        }
    }

    fn pop_removal_event(&mut self) -> Option<Self::Handle>{
        self.removed.pop()
    }
}