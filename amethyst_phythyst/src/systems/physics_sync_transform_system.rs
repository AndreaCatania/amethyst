

use amethyst_core::{
    ecs::{System, ReadExpect, ReadStorage,},
    transform::components::Transform,
};

use crate::Physics;

pub struct PhysicsSyncTransformSystem{
    c: i32
}

impl PhysicsSyncTransformSystem{
    pub fn new() -> PhysicsSyncTransformSystem {
        PhysicsSyncTransformSystem{
            c: 0
        }
    }
}

impl<'a> System<'a> for PhysicsSyncTransformSystem{
    type SystemData = (
        ReadExpect<'a, Physics>,
        ReadStorage<'a, Transform>,
    );

    define_setup_with_physics_assertion!();

    fn run(&mut self, (physics, transform): Self::SystemData){
        physics.server.create_world();
        println!("Sync {}", self.c);
        self.c += 1;
    }
}