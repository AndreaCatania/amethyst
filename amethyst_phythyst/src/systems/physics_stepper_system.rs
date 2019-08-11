use amethyst_core::{
    ecs::{Dispatcher, Read, ReadExpect, RunNow, System, WriteExpect},
    shred::Resources,
    Time,
};

use crate::{objects::*, servers::WorldPhysicsServer, PhysicsTime};

pub struct PhysicsStepperSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
}

impl<N: crate::PtReal> PhysicsStepperSystem<N> {
    pub fn new() -> PhysicsStepperSystem<N> {
        PhysicsStepperSystem {
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsStepperSystem<N> {
    type SystemData = (
        ReadExpect<'a, PhysicsTime>,
        ReadExpect<'a, PhysicsHandle<PhysicsWorldTag>>,
        WriteExpect<'a, WorldPhysicsServer<N>>,
    );

    fn run(&mut self, (physics_time, physics_world, mut world_server): Self::SystemData) {
        world_server.step(physics_world.get(), physics_time.sub_step_seconds.into());
    }
}
