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

/// To use the `Phythyst` crate is necessary to register the `PhysicsBundle` as show below.
///
/// ```rust
/// use amethyst::phythyst::PhysicsBundle;
/// use amethyst::amethyst_nphysics::NPhysicsBackend;
///
/// let game_data = GameDataBuilder::default()
///     .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
///
/// ```
/// Is it possible to define the Physics Engine floating point precision and the [PhysicsBackend](./trait.PhysicsBackend.html);
/// additionally, the physics frame rate can be specified using the function `with_frames_per_seconds`.
///
/// # Dispatcher pipeline
///
/// ##### Behind the scenes
/// To have a stable execution, the physics stepping is executed with a constant frame rate;
/// and to be frame rate agnostic it keep tracks of the elapsed time.
/// **But don't worry**, the above statement means that a physics step can occur multiple times per
/// each frame.
/// So, when you have a `System` that interact with the physics, you have to register it using
/// the API provided by the `PhysicsBundle`; `Phythyst` will take care to execute your `System`s
/// at the right time.
///
/// ##### Pipeline sections
/// The physics pipeline is composed by three sections:
/// - **Pre physics** `with_pre_physics`
///     Executed just before any physics step. In this section of the pipeline you want to register
///     any `System` that will alter the simulation (like add a force or change a transform).
/// - **In physics** `with_in_physics`
///     The `System`s in this stage are executed in parallel with the physics stepping, and this section
///     is meant for all the `System`s that have to be executed each physics frame but doesn't depend
///     on its state.
/// - **Post physics** `with_post_physics`
///     The last section of the physics pipeline, is simply executed just after the physics stepping.
///     In this section, you want to register the `System`s that collects the physics states,
///     (like checking for volumes overlaps, or collision events).
///
/// # Parallel physics dispatching
/// `Phythyst` is designed to dispatch the physics in parallel with everything else, by default.
/// When you start to interact with it, you have to approach it correctly to maintain this property.
///
/// Some internal parts are being explained, and if the physics of your game is not so heavy, or you
/// are not yet confortable with `phythyst`, you can just skip this section.
///
/// The physics pipeline, just explained above, groups all the `System`s that interact with the physics.
/// We can consider all these `System`s, a single group; let's call it `PhysicsBatch`.
/// Like any other `System` in `Amethyst`, the `PhysicsBatch` is dispatched by `shred`, this mean that
/// if we make sure that its resources are not used by any other `System`, registered after it, them will
/// run in parallel.
///
/// ##### Synchronization
/// The main concept is easy, but let's see what it mean in practice.
///
/// When nothing is registered in the `PhysicsBatch`, the only resource that can potentially cause problems
/// is the [Transform Component].
/// To avoid using the [Transform Component] inside the `PhysicsBatch`; `Phythyst` defines the
/// `PhysicsSyncSystem`, that executed at the begining of each frame, it will take care to copy the
/// transforms from the physics to `Amethyst`. Leaving the physics and the rendering untied and free
/// to be executed in parallel.
///
/// The dispatcher looks like this:
/// ```ignore
/// |--Sync--||-------------PhysicsBatch------------|
///           |--Any other System--||-- Rendering --|
/// ```
///
/// Taking as example a race game, you may want to display a scratch on the car when it hits something.
/// To ensure that the physics runs in parallel, you want to register the `System` that checks for the
/// collision, before the `PhysicsBatch` (similarly as was explained above).
///
/// The dispatcher looks like this:
/// ```ignore
/// |--Sync--|         |-------------PhysicsBatch------------|
/// |--CollisionSync--||--Any other System--||-- Rendering --|
/// ```
///
/// That's it.
///
/// ## Small TODO to highlight
/// I'm confident that this section will be removed ASAP, but for the sake of completeness I've to
/// mention a problem.
///
/// The above section, which explains how to make the physics runsin parallel, due to a small
/// Amethyst's design problem, is lying.
/// Indeed, is not possible to run the physics and the rendering in parallel, because they
/// are in two different pipelines.
///
/// So the dispatcher looks like:
/// ```ignore
/// |--Sync--|         |-------------PhysicsBatch------------|
/// |--CollisionSync--||--Any other System--|                 |-- Rendering --|
/// ```
///
/// To know more about it, check this: [https://github.com/AndreaCatania/amethyst/issues/2](https://github.com/AndreaCatania/amethyst/issues/2)
///
/// However, I'm confident that this will be solved soon, and for this reason the above section is
/// written as if this problem doesn't exist.
///
/// [Transform component]: ../amethyst_core/transform/components/struct.Transform.html
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

    pub fn with_frames_per_seconds(mut self, frames_per_seconds: u32) -> Self {
        self.physics_time.set_frames_per_seconds(frames_per_seconds);
        self
    }

    pub fn set_frames_per_seconds(mut self, frames_per_seconds: u32) {
        self.physics_time.set_frames_per_seconds(frames_per_seconds);
    }

    pub fn with_max_sub_steps(mut self, max_sub_steps: u32) -> Self {
        self.physics_time.set_max_sub_steps(max_sub_steps);
        self
    }

    pub fn set_max_sub_steps(mut self, max_sub_steps: u32) {
        self.physics_time.set_max_sub_steps(max_sub_steps);
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
