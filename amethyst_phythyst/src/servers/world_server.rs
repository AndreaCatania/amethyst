
use crate::StoreTag;

/// The world server interface
/// This contains all functionalities to manipulate the world.
pub trait WorldServer{
    fn create_world(&mut self) -> StoreTag;
    fn drop_world(&mut self, world: StoreTag);
}
