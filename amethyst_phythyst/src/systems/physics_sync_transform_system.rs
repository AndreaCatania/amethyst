

use amethyst_core::{
    ecs::{System, ReadExpect, ReadStorage, WriteExpect,},
    transform::components::Transform,
};

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
        ReadStorage<'a, Transform>,
    );

    define_setup_with_physics_assertion!();

    fn run(&mut self, (transform): Self::SystemData){
        println!("Sync {}", self.c);
        self.c += 1;
    }
}