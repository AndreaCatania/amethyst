
use amethyst_core::ecs::{Resources, System, WriteExpect, Write};
use crate::{
    Physics,
    PhysicsWorld,
    PhysicsTime,
    servers::WorldServer,
};
use log::debug;

pub struct PhysicsCommandExecutorSystem;

impl PhysicsCommandExecutorSystem {
    pub fn new() -> Self {
        PhysicsCommandExecutorSystem {}
    }

    fn setup_2(&mut self, res: &mut Resources){
        let mut physics = res.get_mut::<Physics>().unwrap();
        let world = PhysicsWorld(physics.world_server.create_world());
        res.insert(world);
        if !res.has_value::<PhysicsTime>() {
            res.insert(PhysicsTime::default());
        }
        debug!("The physics world is created");
    }
}



impl<'a> System<'a> for PhysicsCommandExecutorSystem {
    type SystemData = (
        WriteExpect<'a, Physics>,
        WriteExpect<'a, PhysicsWorld>,
    );

    define_setup_with_physics_assertion!(setup_2);

    fn run(&mut self, (mut physics, mut world): Self::SystemData){

        // Delayed command execution

    }
}

