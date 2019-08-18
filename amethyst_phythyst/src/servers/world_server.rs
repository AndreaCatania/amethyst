use crate::objects::*;

/// This is the interface that contains all functionalities to manipulate the world.
/// The object that implement this interface is wrapped by `WorldPhysicsServer`.
/// It's stored as resource in the world.
pub trait WorldPhysicsServerTrait<N> {
    /// Create the World and return its Handle.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the world is Dropped automatically.
    fn create_world(&self) -> PhysicsHandle<PhysicsWorldTag>;

    /// This function is responsible to perform the stepping of the world.
    /// This must be called at a fixed rate
    fn step(&self, world: PhysicsWorldTag, delta_time: N);
}
