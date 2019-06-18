
use amethyst_core::{
    Time,
    ecs::{System, WriteExpect, ReadExpect}
};
use crate::{
        Physics,
        PhysicsWorld,
        servers::WorldServer,
};

pub struct PhysicsStepperSystem;

impl PhysicsStepperSystem {
    pub fn new() -> PhysicsStepperSystem{
        PhysicsStepperSystem{}
    }
}

impl<'a> System<'a> for PhysicsStepperSystem{

    type SystemData = (
        ReadExpect<'a, Time>,
        WriteExpect<'a, Physics>,
        ReadExpect<'a, PhysicsWorld>,
    );

    define_setup_with_physics_assertion!();
    
    fn run(&mut self, (time, mut physics, physics_world): Self::SystemData){
        physics.world_server.step_world(physics_world.0, time.delta_real_seconds());
    }

}