
use log::debug;
use amethyst_core::{
    bundle::SystemBundle,
    ecs::DispatcherBuilder,
};
use amethyst_error::Error;

use crate::{
    systems::{
        PhysicsSyncTransformSystem,
        PhysicsStepperSystem,
    },
};

/// This bundle will register all required systems to handle the physics in your game.
/// 
/// This bundle will automatically place a barrier in the dispatcher, in this way
/// allow you to control the systems that you want to execute before and after the
/// synchronization phase just by registering them before or after this bundle.
/// Usually this is the first bundle that you want to register.
pub struct PhysicsBundle;

impl PhysicsBundle{
    pub fn new() -> Self {
        Self{}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error>{

        builder.add(PhysicsSyncTransformSystem::new(), "physics_sync_transform_system", &[]);
        builder.add_barrier();
        builder.add(PhysicsStepperSystem::new(), "", &["physics_sync_transform_system"]);

        debug!("Physics bundle registered.");

        Ok(())
    }
}

