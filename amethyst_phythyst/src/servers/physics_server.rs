

/// The world handle to refer to a particular world
pub struct WorldHandle(pub usize);

/// The body handle to refer to a particular body
pub struct BodyHandle(pub usize);

pub trait PhysicsServer{
    fn create_world(&self) -> WorldHandle;
}

