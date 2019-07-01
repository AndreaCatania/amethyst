use crate::objects::*;

/// This is the interface that contains all functionalities to manipulate the world.
/// The object that implement this interface is wrapped by `WorldPhysicsServer`.
/// It's stored as resource in the world.
pub trait WorldPhysicsServerTrait<N> {
    fn create_world(&mut self) -> PhysicsWorldTag;
    fn drop_world(&mut self, world: PhysicsWorldTag);

    /// This function is responsible to perform the stepping of the world.
    /// This must be called at a fixed rate
    fn step(&self, world: PhysicsWorldTag, delta_time: N);
}
