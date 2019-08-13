use generational_arena::{
    Index, Arena, Iter, IterMut
};

pub type StoreKey = Index;

/// This struct is used to store the physics resources, and return an opaque handle that allow to
/// return a reference to them.
pub struct Storage<T> {
    memory: Arena<T>,
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
    /// This function takes also the ownership, so to drop an object you need to call the `drop`
    /// function with the ID of the object to delete.
    // TODO rename to insert
    pub fn make_opaque(&mut self, object: T) -> StoreKey {
        // Reserve the memory if no more space
        if self.memory.len() == self.memory.capacity() {
            self.memory.reserve(self.growing_size);
        }

        self.memory.insert(object)
    }

    pub fn has(&self, key: StoreKey) -> bool {
        self.memory.contains(key)
    }

    pub fn get(&self, key: StoreKey) -> Option<&T> {
        self.memory.get(key)
    }

    pub fn get_mut(&mut self, key: StoreKey) -> Option<&mut T> {
        self.memory.get_mut(key)
    }

    /// Destroy an object and release the key for future use.
    // TODO rename to drop
    pub fn destroy(&mut self, key: StoreKey) {
        self.memory.remove(key);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.memory.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.memory.iter_mut()
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Storage::new(10, 10)
    }
}
