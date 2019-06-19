
use log::debug;
use amethyst_core::{
    bundle::SystemBundle,
    ecs::DispatcherBuilder,
};
use amethyst_error::Error;

use crate::{
    systems::{
        PhysicsCommandExecutorSystem,
        PhysicsSyncTransformSystem,
        PhysicsStepperSystem,
        PhysicsDispatcherManagerSystem,
    },
    PhysicsDispatcherCreator,
};

/// This bundle will register all required systems to handle the physics in your game.
/// 
/// This bundle will automatically place a barrier in the dispatcher, in this way
/// allow you to control the systems that you want to execute before and after the
/// synchronization phase just by registering them before or after this bundle.
/// Usually this is the first bundle that you want to register.
pub struct PhysicsBundle<G: PhysicsDispatcherCreator + Sync + Send>{
    graph_creator: G,
}

impl<G: PhysicsDispatcherCreator + Sync + Send> PhysicsBundle<G>{
    pub fn new(graph_creator: G) -> Self {
        Self{
            graph_creator
        }
    }
}

impl<'a, 'b, G: 'b + PhysicsDispatcherCreator + Sync + Send> SystemBundle<'a, 'b> for PhysicsBundle<G>{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error>{

        builder.add(PhysicsCommandExecutorSystem::new(), "physics_command_executor_system", &[]);
        builder.add(PhysicsSyncTransformSystem::new(), "physics_sync_transform_system", &["physics_command_executor_system"]);
        builder.add_barrier();
        builder.add(PhysicsStepperSystem::new(), "", &["physics_sync_transform_system"]);
        builder.add_thread_local(PhysicsDispatcherManagerSystem::new(self.graph_creator));

        debug!("Physics bundle registered.");

        Ok(())
    }
}

