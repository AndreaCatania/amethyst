use amethyst_core::{
    ecs::{Dispatcher, Read, ReadExpect, RunNow, System, WriteExpect},
    shred::Resources,
    Time,
};

use crate::{objects::*, servers::WorldPhysicsServer, PhysicsTime};

pub struct PhysicsStepperSystem<N: crate::PhysicsReal> {
    phantom_data: std::marker::PhantomData<N>,
}

impl<N: crate::PhysicsReal> PhysicsStepperSystem<N> {
    pub fn new() -> PhysicsStepperSystem<N> {
        PhysicsStepperSystem {
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, N: crate::PhysicsReal> System<'a> for PhysicsStepperSystem<N> {
    type SystemData = (
        ReadExpect<'a, Time>,
        WriteExpect<'a, PhysicsTime>,
        ReadExpect<'a, PhysicsHandle<PhysicsWorldTag>>,
        WriteExpect<'a, WorldPhysicsServer<N>>,
    );

    define_setup_with_physics_assertion!();

    fn run(&mut self, (time, mut physics_time, physics_world, mut world_server): Self::SystemData) {
        // TODO please remove this once shred allow batch processing
        world_server.consume_events();

        physics_time._time_bank += time.delta_seconds();

        // Avoid spiral performance degradation
        physics_time._time_bank = physics_time._time_bank.min(physics_time._max_bank_size);

        while physics_time._time_bank >= physics_time.sub_step_seconds {
            physics_time._time_bank -= physics_time.sub_step_seconds;

            // TODO start dispatcher

            world_server.step(physics_world.get(), physics_time.sub_step_seconds.into());
        }
    }
}
