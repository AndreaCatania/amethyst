
use slab::{Slab};
use amethyst_phythyst::StoreTag;

/// This struct is used to store the physics resources, and return an opaque handle that allow to
/// return a reference to them.
pub struct Storage<T>{
    memory: Slab<T>,
    growing_size: usize,
}

impl<T> Storage<T>{

    /// Create a storage with an initial capacity
    /// The parameter `growing_size` is used to grow the internal storage by a certain amount when it
    /// hits maximum capacity.
    /// The `growing_size` must be big enough to avoid too much reallocation
    pub fn new(initial_capacity: usize, growing_size: usize) -> Storage<T>{
        Storage {
            memory: Slab::with_capacity(initial_capacity),
            growing_size,
        }
    }

    /// Takes an object and returns an opaque id.
    /// This function takes also the ownership, so to drop an object you need to call the `drop`
    /// function with the ID of the object to delete.
    pub fn make_opaque(&mut self, object : T) -> StoreTag {

        // Reserve the memory if no more space
        if self.memory.len() == self.memory.capacity() {
            self.memory.reserve(self.growing_size);
        }

        let key = self.memory.insert(object);
        TagMachine::make(key + 1)
    }

    pub fn has(&self, tag: StoreTag) -> bool {
        self.memory.contains(TagMachine::read(tag) - 1 )
    }

    pub fn get(&self, tag: StoreTag) -> Option<&T> {
        self.memory.get(TagMachine::read(tag) - 1)
    }


    pub fn get_mut(&mut self, tag: StoreTag) -> Option<&mut T> {
        self.memory.get_mut(TagMachine::read(tag) - 1)
    }

    /// Destroy an object and release the SoreTag for future use.
    pub fn drop(&mut self, tag: StoreTag){
        let object = self.memory.remove(TagMachine::read(tag) - 1 );
        drop(object );
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self{
        Storage::new(10, 10)
    }
}


struct TagMachine;
impl TagMachine{
    pub fn make(id: usize) -> StoreTag {
        StoreTag(std::num::NonZeroUsize::new(id).unwrap())
    }

    pub fn read(tag: StoreTag) -> usize {
        tag.0.get()
    }
}