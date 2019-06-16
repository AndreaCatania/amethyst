
use std::vec::Vec;

/// This struct is used to handle the memory of the handles
/// 
/// ```
/// #[derive(Debug)]
/// struct Data{
///     x: f32,
///     y: i32
/// }
/// 
/// struct Address(std::num::NonZeroUsize);
///     
/// fn main(){
///     
///     let address: Address;
///     
///     {
///         let data = Box::new(Data{x:10.0, y:10});
///         let pointer = Box::into_raw(data);
///         
///         address = Address(std::num::NonZeroUsize::new(pointer as usize).unwrap());
///     }
///     
///     dbg!(address.0);
///     let d : &Data;
///     unsafe{
///         let data = address.0.get() as *const Data;
///         dbg!(&*data);
///         d = &*data;
///     }
///     
///     dbg!(d);
/// }
/// ```
pub struct Memory<T>{
    memory: Vec<*mut T>
}

#[derive(Copy, Clone)]
pub struct ObjId(std::num::NonZeroUsize);

impl<T> Memory<T>{
    pub fn new() -> Memory<T>{
        Memory{
            memory: Vec::new(),
        }
    }

    pub fn make_opaque(&mut self, object : Box<T>) -> ObjId {
        let pointer = Box::into_raw(object);
        
        self.memory.push(pointer);

        ObjId(std::num::NonZeroUsize::new(pointer as usize).unwrap())
    }

    fn is_owner(&self, object_id: ObjId) -> bool {
        false
    }

    pub fn get(&self, object_id: ObjId) -> *const T {
        // TODO Perform this check only in debug mode
        assert!(self.is_owner(object_id));
        unsafe{
            object_id.0.get() as *const T
        }
    }

    pub fn get_mut(&self, object_id: ObjId) -> *mut T {
        // TODO Perform this check only in debug mode
        assert!(self.is_owner(object_id));
        unsafe{
            object_id.0.get() as *mut T
        }
    }

    pub fn drop(&self, object_id: ObjId){
        // TODO Perform this check only in debug mode
        assert!(self.is_owner(object_id));
        unsafe{
            let pointer = object_id.0.get() as *mut T;
            self.memory.remove_item(&*pointer);
            let boxed = Box::from_raw(pointer);
            drop(boxed);
        }
    }
}