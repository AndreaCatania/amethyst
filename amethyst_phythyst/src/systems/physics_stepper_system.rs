
use amethyst_core::{
    Time,
    ecs::{System, RunNow, WriteExpect, ReadExpect, Read, Dispatcher,},
    shred::Resources,
};
use crate::{
    PhysicsWorldServer,
    PhysicsTime,
    servers::{
        PhysicsWorldTag,
        WorldServer,
    },
};


pub struct PhysicsStepperSystem;

impl PhysicsStepperSystem {
    pub fn new() -> PhysicsStepperSystem{
        PhysicsStepperSystem{}
    }
}

impl<'a,> System<'a> for PhysicsStepperSystem{

    type SystemData = (
        ReadExpect<'a, Time>,
        WriteExpect<'a, PhysicsTime>,
        ReadExpect<'a, PhysicsWorldTag>,
        WriteExpect<'a, PhysicsWorldServer>,
    );

    define_setup_with_physics_assertion!();
    
    fn run(&mut self, (time, mut physics_time, physics_world, mut physics_world_server): Self::SystemData){

        physics_time._time_bank += time.delta_seconds();

        // Avoid spiral performance degradation
        physics_time._time_bank = physics_time._time_bank.min(physics_time._max_bank_size);

        while physics_time._time_bank >= physics_time.sub_step_seconds {

            physics_time._time_bank -= physics_time.sub_step_seconds;

            // TODO start dispatcher

            physics_world_server.step(*physics_world, physics_time.sub_step_seconds);
        }
    }

}