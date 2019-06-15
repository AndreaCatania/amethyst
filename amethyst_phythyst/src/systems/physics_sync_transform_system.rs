
use amethyst_core::ecs::System;

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
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData){
        println!("Sync {}", self.c);
        self.c += 1;
    }
}