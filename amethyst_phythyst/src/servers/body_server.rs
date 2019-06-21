
use crate::StoreTag;

/// The body server interface
/// This contains all functionalities to manipulate the body.
pub trait BodyServer{

    fn create(&mut self, world: StoreTag) -> StoreTag;
    fn drop(&mut self, body: StoreTag);

}