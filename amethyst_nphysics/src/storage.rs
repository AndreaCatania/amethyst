use generational_arena::{Arena, Index, Iter, IterMut};

pub type StoreKey = Index;

/// This struct is used to store the physics resources, and return an opaque handle that allow to
/// return a reference to them.
// TODO
//I've a vector that holds some objects. Some algorithms can add and remove these objects at the same time. To do it I'm using Arc<RwLock<Vec<Object>>.
//
//Often happens that I've to modify the objects held by this vector, so I take a mutable reference of this vector (from RwLock) and then I can modify the object that I want.
//The problem is that, if another function want to modify another object, it has to wait that the first function release it.
//
//Use something like Arc<RwLock<Vec<Arc<RwLock<Vec<Object>>>> seems a bit too much, do you know vector structures that are optimized to handle these situations?
//If not, what do you think about -> Arc<RwLock<Vec<Arc<RwLock<Vec<Object>>>> ?
// Another idea is to create a storage that take care of make a thread and simulate a RwLock.
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
    /// This function takes also the ownership, so to drop an object you need to call the `remove`
    /// function with the ID of the object to delete.
    pub fn insert(&mut self, object: T) -> StoreKey {
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

    /// Remove an object and release the key for future use.
    ///
    /// Returns `Some` with the removed object, or `None` if nothing was removed.
    pub fn remove(&mut self, key: StoreKey) -> Option<T> {
        self.memory.remove(key)
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
