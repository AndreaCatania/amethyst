use amethyst_core::{
    bundle::SystemBundle,
    deferred_dispatcher_operation::*,
    ecs::{DispatcherBuilder, ReadStorage, System, SystemData, World},
};
use amethyst_error::Error;
use log::info;

use crate::{
    prelude::*,
    servers::PhysicsServers,
    systems::{
        PhysicsBatchSystem, PhysicsStepperSystem, PhysicsSyncShapeSystem,
        PhysicsSyncTransformSystem,
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
pub struct PhysicsBundle<'a, 'b, N: crate::PtReal, B: crate::PhysicsBackend<N>> {
    phantom_data_float: std::marker::PhantomData<N>,
    phantom_data_backend: std::marker::PhantomData<B>,
    physics_time: PhysicsTime,
    physics_builder: DispatcherBuilder<'a, 'b>,
    pre_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    in_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    post_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

macro_rules! define_setters{
    ($with:ident, $add:ident, $vec:ident) => {
        pub fn $with<S>(
            mut self,
            system: S,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) -> Self
        where
            S: for<'c> System<'c> + 'static + Send,
        {
            self.add_pre_physics(system, name, dependencies);
            self
        }

        pub fn $add<S>(
            &mut self,
            system: S,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) where
            S: for<'c> System<'c> + 'static + Send,
        {
            self.$vec
                .push(Box::new(AddSystem {
                    system,
                    name,
                    dependencies,
                }) as Box<dyn DispatcherOperation<'a, 'b>>);
        }
    }
}

impl<'a, 'b, N: crate::PtReal, B: crate::PhysicsBackend<N>> PhysicsBundle<'a, 'b, N, B> {
    pub fn new() -> Self {
        Self {
            phantom_data_float: std::marker::PhantomData,
            phantom_data_backend: std::marker::PhantomData,
            physics_time: PhysicsTime::default(),
            physics_builder: DispatcherBuilder::new(),
            pre_physics_dispatcher_operations: Vec::new(),
            in_physics_dispatcher_operations: Vec::new(),
            post_physics_dispatcher_operations: Vec::new(),
        }
    }

    pub fn with_frames_per_second(mut self, frames_per_second: u32) -> Self {
        self.physics_time.set_frames_per_second(frames_per_second);
        self
    }

    pub fn set_frames_per_second(mut self, frames_per_second: u32) {
        self.physics_time.set_frames_per_second(frames_per_second);
    }

    pub fn with_max_sub_steps(mut self, frames_per_second: u32) -> Self {
        self.physics_time.set_max_sub_steps(frames_per_second);
        self
    }

    pub fn set_max_sub_steps(mut self, frames_per_second: u32) {
        self.physics_time.set_max_sub_steps(frames_per_second);
    }

    define_setters!(
        with_pre_physics,
        add_pre_physics,
        pre_physics_dispatcher_operations
    );
    define_setters!(
        with_in_physics,
        add_in_physics,
        in_physics_dispatcher_operations
    );
    define_setters!(
        with_post_physics,
        add_post_physics,
        post_physics_dispatcher_operations
    );
}

type PhysicsSetupStorages<'a> = (
    ReadStorage<'a, PhysicsHandle<PhysicsWorldTag>>,
    ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
    ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
    ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
);

impl<N, B> SystemBundle<'static, 'static> for PhysicsBundle<'static, 'static, N, B>
where
    N: crate::PtReal,
    B: crate::PhysicsBackend<N> + Send + 'static,
{
    fn build(
        mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'static, 'static>,
    ) -> Result<(), Error> {
        PhysicsSetupStorages::setup(world);

        {
            let (mut world_server, rb_server, area_server, shape_server) = B::create_servers();
            let physics_world = world_server.create_world();
            world.insert(world_server);
            world.insert(rb_server);
            world.insert(area_server);
            world.insert(shape_server);
            world.insert(self.physics_time);
            world.insert(physics_world);
        }

        let mut physics_builder = self.physics_builder;

        // Register PRE physics operations
        self.pre_physics_dispatcher_operations
            .into_iter()
            .try_for_each(|operation| operation.exec(world, &mut physics_builder))
            .unwrap_or_else(|e| panic!("Failed to setup the pre physics systems. Error: {}", e));

        // Register IN physics operations
        physics_builder.add_barrier();
        physics_builder.add(PhysicsStepperSystem::<N>::new(), "", &[]);
        self.in_physics_dispatcher_operations
            .into_iter()
            .try_for_each(|operation| operation.exec(world, &mut physics_builder))
            .unwrap_or_else(|e| panic!("Failed to setup the in physics systems. Error: {}", e));

        // Register POST physics operations
        physics_builder.add_barrier();
        self.post_physics_dispatcher_operations
            .into_iter()
            .try_for_each(|operation| operation.exec(world, &mut physics_builder))
            .unwrap_or_else(|e| panic!("Failed to setup the post physics systems. Error: {}", e));

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
        builder.add_batch::<PhysicsBatchSystem<N>>(
            physics_builder,
            "physics_batch",
            &["physics_sync_entity", "physics_sync_transform"],
        );

        info!("Physics bundle registered.");

        Ok(())
    }
}
