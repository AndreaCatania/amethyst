
use amethyst_core::ecs::{System};

pub struct PhysicsStepperSystem{
    c: i32
}

impl PhysicsStepperSystem {
    pub fn new() -> PhysicsStepperSystem{
        PhysicsStepperSystem{
            c:0
        }
    }
}

impl<'a> System<'a> for PhysicsStepperSystem{

    type SystemData = ();

    define_setup_with_physics_assertion!();
    
    fn run(&mut self, data: Self::SystemData){
        println!("Stepper {}", self.c);
        self.c +=1;
    }

}