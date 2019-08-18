use amethyst_phythyst::{objects::PhysicsBodyTag, PtReal};
use generational_arena::{Iter, IterMut};
use nphysics3d::object::{Body as NpBody, BodySet};

use crate::{
    body::Body,
    storage::{Storage, StoreKey},
};

pub struct BodyStorage<N: PtReal> {
    storage: Storage<Box<Body<N>>>,
    /// A list of removed ID, this list is decremented only when the function `pop_removal_event` is called
    removed: Vec<StoreKey>,
}

impl<N: PtReal> BodyStorage<N> {
    pub fn new() -> Self {
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

impl<N: PtReal> BodyStorage<N> {
    pub fn insert_body(&mut self, body: Box<Body<N>>) -> StoreKey {
        self.storage.insert(body)
    }

    pub fn drop_body(&mut self, key: StoreKey) {
        self.storage.remove(key);
        self.removed.push(key);
    }

    pub fn get_body(&self, key: StoreKey) -> Option<&Box<Body<N>>> {
        self.storage.get(key)
    }

    pub fn get_body_mut(&mut self, key: StoreKey) -> Option<&mut Box<Body<N>>> {
        self.storage.get_mut(key)
    }

    pub fn iter(&self) -> Iter<'_, Box<Body<N>>> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<Body<N>>> {
        self.storage.iter_mut()
    }
}

impl<N: PtReal> BodySet<N> for BodyStorage<N> {
    type Body = dyn NpBody<N>;
    type Handle = StoreKey;

    fn get(&self, handle: Self::Handle) -> Option<&Self::Body> {
        if let Some(body) = self.storage.get(handle) {
            Some(body.np_body.as_ref())
        } else {
            None
        }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::Body> {
        if let Some(body) = self.storage.get_mut(handle) {
            Some(body.np_body.as_mut())
        } else {
            None
        }
    }

    fn get_pair_mut(
        &mut self,
        handle1: Self::Handle,
        handle2: Self::Handle,
    ) -> (Option<&mut Self::Body>, Option<&mut Self::Body>) {
        assert_ne!(handle1, handle2, "Both body handles must not be equal.");
        let b1 = self.get_mut(handle1).map(|b| b as *mut dyn NpBody<N>);
        let b2 = self.get_mut(handle2).map(|b| b as *mut dyn NpBody<N>);
        unsafe {
            use std::mem;
            (b1.map(|b| mem::transmute(b)), b2.map(|b| mem::transmute(b)))
        }
    }

    fn contains(&self, handle: Self::Handle) -> bool {
        self.storage.has(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &Self::Body)) {
        for (h, b) in self.storage.iter() {
            f(h, b.np_body.as_ref())
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut Self::Body)) {
        for (h, b) in self.storage.iter_mut() {
            f(h, b.np_body.as_mut())
        }
    }

    fn pop_removal_event(&mut self) -> Option<Self::Handle> {
        self.removed.pop()
    }
}
