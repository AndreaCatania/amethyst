use std::{
    cell::UnsafeCell,
    sync::{Mutex, MutexGuard},
};

use generational_arena::{Arena, Index, Iter, IterMut};

pub type StoreKey = Index;

/// This struct is used to store the physics resources, and return an opaque handle that allow to
/// return a reference to them.
///
/// Each value is protected by a Mutex that allow parallel
pub struct Storage<T> {
    memory: Arena<(UnsafeCell<T>, Mutex<()>)>,
    growing_size: usize,
}

impl<T> Storage<T> {
    /// Create a storage with an initial capacity
    /// The parameter `growing_size` is used to grow the internal storage by a certain amount when it
    /// hits maximum capacity.
    /// The `growing_size` must be big enough to avoid too much reallocation
    pub fn new(initial_capacity: usize, growing_size: usize) -> Storage<T> {
        Storage {
            memory: Arena::with_capacity(initial_capacity),
            growing_size,
        }
    }

    /// Takes an object and returns an opaque id.
    /// This function takes also the ownership, so to drop an object you need to call the `remove`
    /// function with the ID of the object to delete.
    pub fn insert(&mut self, object: T) -> StoreKey {
        // Reserve the memory if no more space
        if self.memory.len() == self.memory.capacity() {
            self.memory.reserve(self.growing_size);
        }

        self.memory
            .insert((UnsafeCell::new(object), Mutex::new(())))
    }

    pub fn has(&self, key: StoreKey) -> bool {
        self.memory.contains(key)
    }

    pub fn get(&self, key: StoreKey) -> Option<&T> {
        unsafe { self.memory.get(key).map(|v| &*v.0.get()) }
    }

    pub fn get_mut(&self, key: StoreKey) -> Option<StorageGuard<T>> {
        unsafe {
            self.memory.get(key).map(|v| StorageGuard {
                data: &mut *v.0.get(),
                _guard: v.1.lock().unwrap(),
            })
        }
    }

    pub fn mut_get_mut(&mut self, key: StoreKey) -> Option<&mut T> {
        unsafe { self.memory.get(key).map(|v| &mut *v.0.get()) }
    }

    /// Remove an object and release the key for future use.
    ///
    /// Returns `Some` with the removed object, or `None` if nothing was removed.
    pub fn remove(&mut self, key: StoreKey) -> Option<T> {
        self.memory.remove(key).map(|v| v.0.into_inner())
    }

    pub fn iter(&self) -> Iter<'_, (UnsafeCell<T>, Mutex<()>)> {
        self.memory.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (UnsafeCell<T>, Mutex<()>)> {
        self.memory.iter_mut()
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Storage::new(10, 10)
    }
}

unsafe impl<T> Sync for Storage<T> {}

pub struct StorageGuard<'a, T> {
    data: &'a mut T,
    _guard: MutexGuard<'a, ()>,
}

impl<T> std::ops::Deref for StorageGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> std::ops::DerefMut for StorageGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
