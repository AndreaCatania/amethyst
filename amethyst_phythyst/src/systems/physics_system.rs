use amethyst_core::ecs::{prelude::*, storage::ComponentEvent, ReaderId};

use crate::{prelude::*, PhysicsTime};

/// Used only to initialize the physics resources.
pub struct PhysicsSystem<N: amethyst_core::math::RealField> {
    servers: Option<PhysicsServers<N>>,
}

impl<N: amethyst_core::math::RealField> PhysicsSystem<N> {
    pub fn new(servers: PhysicsServers<N>) -> Self {
        PhysicsSystem {
            servers: Some(servers),
        }
    }
}

impl<'a, N: amethyst_core::math::RealField> System<'a> for PhysicsSystem<N> {
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

        let (mut world_server, rb_server, area_server, shape_server) = self.servers.take().unwrap();

        let physics_world = world_server.create_world();

        res.insert(world_server);
        res.insert(rb_server);
        res.insert(area_server);
        res.insert(shape_server);
        res.insert(PhysicsTime::default());
        res.insert(physics_world);
    }
}
