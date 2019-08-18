use amethyst_phythyst::{objects::PhysicsBodyTag, PtReal};
use nphysics3d::{
    joint::{JointConstraint as NpJointConstraint, JointConstraintSet as NpJointConstraintSet},
    object::{BodyPartHandle as NpBodyPartHandle, BodySet as NpBodySet},
};

use crate::{
    joint::Joint,
    storage::{Storage, StoreKey},
};

pub struct JointStorage<N: PtReal, S: NpBodySet<N>> {
    storage: Storage<Box<Joint<N, S>>>,
    /// A list of inserted ID, this list is decremented only when the function `pop_inserted_event` is called
    inserted: Vec<(
        StoreKey,
        NpBodyPartHandle<S::Handle>,
        NpBodyPartHandle<S::Handle>,
    )>,
    /// A list of removed ID, this list is decremented only when the function `pop_removal_event` is called
    removed: Vec<(
        StoreKey,
        NpBodyPartHandle<S::Handle>,
        NpBodyPartHandle<S::Handle>,
    )>,
}

impl<N: PtReal, S: NpBodySet<N>> JointStorage<N, S> {
    pub fn new() -> Self {
        JointStorage {
            storage: Storage::new(5, 15),
            inserted: Vec::new(),
            removed: Vec::new(),
        }
    }
}

impl<N: PtReal, S: NpBodySet<N>> Default for JointStorage<N, S> {
    fn default() -> Self {
        JointStorage::new()
    }
}

impl<N: PtReal, S: NpBodySet<N>> JointStorage<N, S> {
    pub fn insert(&mut self, joint: Box<Joint<N, S>>) -> StoreKey {
        let (part1, part2) = joint.np_joint.anchors();
        let key = self.storage.insert(joint);
        self.inserted.push((key, part1, part2));
        key
    }

    pub fn drop(&mut self, key: StoreKey) {
        let res = self.storage.remove(key);
        if let Some(data) = res {
            let (part1, part2) = data.np_joint.anchors();
            self.inserted.push((key, part1, part2));
        }
    }

    pub fn get_collider(&self, key: StoreKey) -> Option<&Box<Joint<N, S>>> {
        self.storage.get(key)
    }

    pub fn get_collider_mut(&mut self, key: StoreKey) -> Option<&mut Box<Joint<N, S>>> {
        self.storage.get_mut(key)
    }
}

impl<N: PtReal, S: NpBodySet<N> + 'static> NpJointConstraintSet<N, S> for JointStorage<N, S> {
    type JointConstraint = dyn NpJointConstraint<N, S>;
    type Handle = StoreKey;

    fn get(&self, handle: Self::Handle) -> Option<&Self::JointConstraint> {
        if let Some(joint) = self.storage.get(handle) {
            Some(joint.np_joint.as_ref())
        } else {
            None
        }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::JointConstraint> {
        if let Some(joint) = self.storage.get_mut(handle) {
            Some(joint.np_joint.as_mut())
        } else {
            None
        }
    }

    fn contains(&self, handle: Self::Handle) -> bool {
        self.storage.has(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &Self::JointConstraint)) {
        for (i, c) in self.storage.iter() {
            f(i, c.np_joint.as_ref())
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut Self::JointConstraint)) {
        for (i, c) in self.storage.iter_mut() {
            f(i, c.np_joint.as_mut())
        }
    }

    fn pop_insertion_event(
        &mut self,
    ) -> Option<(
        Self::Handle,
        NpBodyPartHandle<S::Handle>,
        NpBodyPartHandle<S::Handle>,
    )> {
        self.inserted.pop()
    }

    fn pop_removal_event(
        &mut self,
    ) -> Option<(
        Self::Handle,
        NpBodyPartHandle<S::Handle>,
        NpBodyPartHandle<S::Handle>,
    )> {
        self.removed.pop()
    }

    fn remove(&mut self, to_remove: Self::Handle) {
        self.storage.remove(to_remove);
    }
}
