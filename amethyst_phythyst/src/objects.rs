use amethyst_core::ecs::{Component, DenseVecStorage};

/// PhysicsWorld is the opaque ID that identify a world in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsWorldTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsWorldTag {
    fn default() -> Self {
        panic!();
        PhysicsWorldTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsWorldTag {
    type Storage = DenseVecStorage<PhysicsWorldTag>;
}

impl std::ops::Deref for PhysicsWorldTag {
    type Target = std::num::NonZeroUsize;
    fn deref(&self) -> &std::num::NonZeroUsize {
        &self.0
    }
}

/// PhysicsBody is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsBodyTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsBodyTag {
    fn default() -> Self {
        panic!();
        PhysicsBodyTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl Component for PhysicsBodyTag {
    type Storage = DenseVecStorage<PhysicsBodyTag>;
}

impl std::ops::Deref for PhysicsBodyTag {
    type Target = std::num::NonZeroUsize;
    fn deref(&self) -> &std::num::NonZeroUsize {
        &self.0
    }
}

/// Physics shape is the opaque ID that identify a body in the physics server
#[derive(Copy, Clone)]
pub struct PhysicsShapeTag(pub std::num::NonZeroUsize);

/// Panic if called
impl Default for PhysicsShapeTag {
    fn default() -> Self {
        panic!();
        PhysicsShapeTag(std::num::NonZeroUsize::new(1).unwrap())
    }
}

impl std::ops::Deref for PhysicsShapeTag {
    type Target = std::num::NonZeroUsize;
    fn deref(&self) -> &std::num::NonZeroUsize {
        &self.0
    }
}
