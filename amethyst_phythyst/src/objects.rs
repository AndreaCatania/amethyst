

use amethyst_core::ecs::{DenseVecStorage, Component};

/// PhysicsWorld is the opaque ID that identify a world in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsWorldTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsWorldTag{
    fn default() -> Self {
        panic!();
        PhysicsWorldTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsWorldTag{
    type Storage = DenseVecStorage<PhysicsWorldTag>;
}

/// PhysicsBody is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsBodyTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsBodyTag{
    fn default() -> Self {
        panic!();
        PhysicsBodyTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsBodyTag{
    type Storage = DenseVecStorage<PhysicsBodyTag>;
}