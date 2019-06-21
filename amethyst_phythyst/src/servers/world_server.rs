
use crate::StoreTag;

/// The world server interface
/// This contains all functionalities to manipulate the world.
pub trait WorldServer{
    fn create(&mut self) -> StoreTag;
    fn drop(&mut self, world: StoreTag);

    fn step(&mut self, world: StoreTag, delta_time: f32);

    //fn add_body(&mut self, world: StoreTag, body: StoreTag );
}
