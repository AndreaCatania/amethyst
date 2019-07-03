use crate::{objects::*, servers::RBodyPhysicsServer};
use amethyst_core::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    transform::components::Transform,
};

pub struct PhysicsSyncTransformSystem {
}

impl PhysicsSyncTransformSystem {
    pub fn new() -> PhysicsSyncTransformSystem{
        PhysicsSyncTransformSystem {  }
    }
}

impl<'a> System<'a> for PhysicsSyncTransformSystem {
    type SystemData = (
        ReadExpect<'a, RBodyPhysicsServer<f32>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsBodyTag>,
    );

    define_setup_with_physics_assertion!();

    fn run(&mut self, (rbody_server, mut transforms, bodies): Self::SystemData) {
        for (transform, rb) in (&mut transforms, &bodies).join() {
            // TODO please avoid much copies by sending the mutable reference directly
            *transform = rbody_server.body_transform(*rb);
        }
    }
}
