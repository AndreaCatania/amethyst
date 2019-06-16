

/// The world handle to refer to a particular world
pub struct WorldHandle(pub std::num::NonZeroUsize);

/// The world server interface
/// This contains all functionalities to manipulate the world
pub trait WorldServer{
    fn create_world(&self) -> WorldHandle;
}
