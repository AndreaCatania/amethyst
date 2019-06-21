
use amethyst_core::ecs::{
    Resources,
    Dispatcher,
    DispatcherBuilder,
};

pub trait PhysicsDispatcherCreator {
    fn rebuild(&mut self, res: &Resources) -> bool;
    fn build<'a, 'b>(&mut self, res: &Resources) -> Option<Dispatcher<'a, 'b>>;
}

pub struct EmptyPhysicsDispatcherCreator;

/// Graph trait implementation required by consumers. Builds a graph and manages signaling when
/// the graph needs to be rebuilt.
///
/// To control the physics objects you must create a dispatcher and register the systems that will
/// processed each sub step.
impl PhysicsDispatcherCreator for EmptyPhysicsDispatcherCreator {

    /// Check if graph needs to be rebuilt.
    /// This function is evaluated every frame before running the graph.
    fn rebuild(&mut self, res: &Resources) -> bool{
        false
    }

    /// Build and return the configured complete physics dispatcher.
    fn build<'a, 'b>(&mut self, res: &Resources) -> Option<Dispatcher<'a, 'b>>{
        DispatcherBuilder::new().
        None
    }
}

impl Default for EmptyPhysicsDispatcherCreator {
    fn default() -> Self {
        EmptyPhysicsDispatcherCreator {}
    }
}

pub(crate) struct PhysicsDispatcher<'a, 'b>(pub Option<Dispatcher<'a, 'b>>);

unsafe impl<'a, 'b> Send for PhysicsDispatcher<'a, 'b>{}
unsafe impl<'a, 'b> Sync for PhysicsDispatcher<'a, 'b>{}
