use amethyst_core::{
    ecs::{prelude::*, storage::ComponentEvent, ReaderId},
};

use crate::{prelude::*, PhysicsTime};

/// Used only to initialize the physics resources.
pub struct PhysicsSystem<N: crate::PhysicsReal, B: crate::PhysicsBackend<N>> {
    phantom_data_float: std::marker::PhantomData<N>,
    phantom_data_backend: std::marker::PhantomData<B>,
}

impl<N: crate::PhysicsReal, B: crate::PhysicsBackend<N>> PhysicsSystem<N, B> {
    pub fn new() -> Self {
        PhysicsSystem {
            phantom_data_float: std::marker::PhantomData,
            phantom_data_backend: std::marker::PhantomData,
        }
    }
}

impl<'a, N: crate::PhysicsReal, B: crate::PhysicsBackend<N>> System<'a> for PhysicsSystem<N, B> {
    // Used only to register the storages.
    type SystemData = (
        ReadStorage<'a, PhysicsHandle<PhysicsWorldTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
    );

    fn run(&mut self, _: Self::SystemData) {}

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        let (mut world_server, rb_server, area_server, shape_server) = B::create_servers();

        let physics_world = world_server.create_world();

        res.insert(world_server);
        res.insert(rb_server);
        res.insert(area_server);
        res.insert(shape_server);
        res.insert(PhysicsTime::default());
        res.insert(physics_world);
    }
}
