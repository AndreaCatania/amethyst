

use amethyst_core::{
    ecs::{System, ReadExpect, WriteStorage, ReadStorage, Join},
    transform::components::Transform,
};
use crate::{
    servers::RBodyPhysicsServer,
    objects::*,
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
        ReadExpect<'a, RBodyPhysicsServer>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsWorldTag>,
        ReadStorage<'a, PhysicsBodyTag>,
    );

    define_setup_with_physics_assertion!();

    fn run(&mut self, (rbody_server, mut transforms, world, bodies): Self::SystemData){


        for (transform, world, rb) in (&mut transforms, &world, &bodies).join() {

            // TODO please avoid much copies by sending the mutable reference directly
            *transform = rbody_server.body_transform(*world, *rb);
        }


    }
}