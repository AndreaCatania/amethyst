use amethyst_core::{
    bundle::SystemBundle,
    ecs::{DispatcherBuilder, ReadStorage, SystemData, World},
};
use amethyst_error::Error;
use log::info;

use crate::{
    prelude::*,
    servers::PhysicsServers,
    systems::{
        PhysicsStepperSystem, PhysicsSyncShapeSystem, PhysicsSyncTransformSystem,
    },
    PhysicsTime,
};

/// This bundle registers the `Phythyst` `System`s that will handle the most tricky and redundant
/// part of the physics engine for you.
///
/// ```rust
/// use amethyst::phythyst::PhysicsBundle;
/// use amethyst::amethyst_nphysics::NPhysicsBackend;
///
/// let game_data = GameDataBuilder::default()
///     .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
///
/// ```
/// During the creation of the `PhysicsBundle` (as show above), is possible to define the floating
/// point precision and the [PhysicsBackend](./trait.PhysicsBackend.html).
///
/// TODO please continue dispathcer pipeline description once implemented
// TODO, this must be converted in PhysicsDispatcherBuilder that accept systems and bundles.
// It will have three stages where i possible register Systems and Bundles.
//  PrePhysics: These Systems are executed always before the physics step.
//  InPhysics: These Systems are executed in parallel with the physics step.
//  PostPhysics: These Systems are executed always after the physics step.
pub struct PhysicsBundle<N: crate::PtReal, B: crate::PhysicsBackend<N>> {
    phantom_data_float: std::marker::PhantomData<N>,
    phantom_data_backend: std::marker::PhantomData<B>,
}

impl<N: crate::PtReal, B: crate::PhysicsBackend<N>> PhysicsBundle<N, B> {
    pub fn new() -> Self {
        Self {
            phantom_data_float: std::marker::PhantomData,
            phantom_data_backend: std::marker::PhantomData,
        }
    }
}

impl<'a, 'b, N, B> SystemBundle<'a, 'b> for PhysicsBundle<N, B>
where
    N: crate::PtReal,
    B: crate::PhysicsBackend<N> + Send + 'a,
{
    fn build(
        mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        {
            ReadStorage::<'a, PhysicsHandle<PhysicsWorldTag>>::setup(world);
            ReadStorage::<'a, PhysicsHandle<PhysicsBodyTag>>::setup(world);
            ReadStorage::<'a, PhysicsHandle<PhysicsAreaTag>>::setup(world);
            ReadStorage::<'a, PhysicsHandle<PhysicsShapeTag>>::setup(world);

            let (mut world_server, rb_server, area_server, shape_server) = B::create_servers();
            let physics_world = world_server.create_world();
            world.insert(world_server);
            world.insert(rb_server);
            world.insert(area_server);
            world.insert(shape_server);
            world.insert(PhysicsTime::default());
            world.insert(physics_world);
        }

        builder.add(
            PhysicsSyncShapeSystem::<N>::default(),
            "physics_sync_entity",
            &[],
        );
        builder.add(
            PhysicsSyncTransformSystem::<N>::new(),
            "physics_sync_transform",
            &[],
        );
        builder.add_barrier();
        builder.add(
            PhysicsStepperSystem::<N>::new(),
            "",
            &["physics_sync_transform"], // TODO Useless since I'm using the barrier
        );

        info!("Physics bundle registered.");

        Ok(())
    }
}
