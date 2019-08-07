use amethyst_core::{bundle::SystemBundle, ecs::DispatcherBuilder};
use amethyst_error::Error;
use log::debug;

use crate::{
    servers::PhysicsServers,
    systems::{
        PhysicsStepperSystem, PhysicsSyncShapeSystem, PhysicsSyncTransformSystem, PhysicsSystem,
    },
};

/// This bundle will register all required systems to handle the physics in your game.
///
/// This bundle will automatically place a barrier in the dispatcher, in this way
/// allow you to control the systems that you want to execute before and after the
/// synchronization phase just by registering them before or after this bundle.
/// Usually this is the first bundle that you want to register.
///
/// TODO, this must be converted in PhysicsDispatcherBuilder that accept systems and bundles.
/// It will have three stages where i possible register Systems and Bundles.
///  PrePhysics: These Systems are executed always before the physics step.
///  InPhysics: These Systems are executed in parallel with the physics step.
///  PostPhysics: These Systems are executed always after the physics step.
pub struct PhysicsBundle<N: amethyst_core::math::RealField> {
    servers: Option<PhysicsServers<N>>,
}

impl<N: amethyst_core::math::RealField> PhysicsBundle<N> {
    pub fn new(servers: PhysicsServers<N>) -> Self {
        Self {
            servers: Some(servers),
        }
    }
}

impl<'a, 'b, N: amethyst_core::math::RealField> SystemBundle<'a, 'b> for PhysicsBundle<N> {
    fn build(mut self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PhysicsSystem::new(self.servers.take().unwrap()), "", &[]);
        builder.add(
            PhysicsSyncShapeSystem::default(),
            "physics_sync_entity",
            &[],
        );
        builder.add(
            PhysicsSyncTransformSystem::new(),
            "physics_sync_transform",
            &[],
        );
        builder.add_barrier();
        builder.add(
            PhysicsStepperSystem::new(),
            "",
            &["physics_sync_transform"], // TODO Useless since I'm using the barrier
        );

        debug!("Physics bundle registered.");

        Ok(())
    }
}
