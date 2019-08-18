use amethyst_phythyst::{objects::PhysicsBodyTag, PtReal};
use nphysics3d::{
    force_generator::{
        ForceGenerator as NpForceGenerator, ForceGeneratorSet as NpForceGeneratorSet,
    },
    object::{BodyPartHandle as NpBodyPartHandle, BodySet as NpBodySet},
};

use crate::{
    force_generator::ForceGenerator,
    storage::{Storage, StoreKey},
};

pub struct ForceGeneratorStorage<N: PtReal, S: NpBodySet<N>> {
    storage: Storage<Box<ForceGenerator<N, S>>>,
}

impl<N: PtReal, S: NpBodySet<N>> ForceGeneratorStorage<N, S> {
    pub fn new() -> Self {
        ForceGeneratorStorage {
            storage: Storage::new(5, 5),
        }
    }
}

impl<N: PtReal, S: NpBodySet<N>> Default for ForceGeneratorStorage<N, S> {
    fn default() -> Self {
        ForceGeneratorStorage::new()
    }
}

impl<N: PtReal, S: NpBodySet<N>> ForceGeneratorStorage<N, S> {
    pub fn insert(&mut self, force_generator: Box<ForceGenerator<N, S>>) -> StoreKey {
        self.storage.make_opaque(force_generator)
    }

    pub fn drop(&mut self, key: StoreKey) {
        self.storage.destroy(key);
    }

    pub fn get_collider(&self, key: StoreKey) -> Option<&Box<ForceGenerator<N, S>>> {
        self.storage.get(key)
    }

    pub fn get_collider_mut(&mut self, key: StoreKey) -> Option<&mut Box<ForceGenerator<N, S>>> {
        self.storage.get_mut(key)
    }
}

impl<N: PtReal, S: NpBodySet<N> + 'static> NpForceGeneratorSet<N, S>
    for ForceGeneratorStorage<N, S>
{
    type ForceGenerator = dyn NpForceGenerator<N, S>;
    type Handle = StoreKey;

    fn get(&self, handle: Self::Handle) -> Option<&Self::ForceGenerator> {
        if let Some(force_generator) = self.storage.get(handle) {
            Some(force_generator.np_force_generator.as_ref())
        } else {
            None
        }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::ForceGenerator> {
        if let Some(force_generator) = self.storage.get_mut(handle) {
            Some(force_generator.np_force_generator.as_mut())
        } else {
            None
        }
    }

    fn contains(&self, handle: Self::Handle) -> bool {
        self.storage.has(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &Self::ForceGenerator)) {
        for (i, c) in self.storage.iter() {
            f(i, c.np_force_generator.as_ref())
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut Self::ForceGenerator)) {
        for (i, c) in self.storage.iter_mut() {
            f(i, c.np_force_generator.as_mut())
        }
    }
}
