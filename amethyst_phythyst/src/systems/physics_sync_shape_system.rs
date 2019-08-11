use amethyst_core::ecs::{prelude::*, storage::ComponentEvent, ReaderId};

use crate::prelude::*;

/// Thanks to this `System`, it is enough to set a shape as a `Component` of an `Entity`, to use it
/// as a rigid body shape.
/// Here, the automatic association of the `Shape` to the `RigidBody` is managed.
pub struct PhysicsSyncShapeSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    shapes_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<N: crate::PtReal> Default for PhysicsSyncShapeSystem<N> {
    fn default() -> Self {
        PhysicsSyncShapeSystem {
            phantom_data: std::marker::PhantomData,
            bodies_event_reader: None,
            shapes_event_reader: None,
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsSyncShapeSystem<N> {
    type SystemData = (
        ReadExpect<'a, RBodyPhysicsServer<N>>,
        ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
    );

    fn run(&mut self, (body_server, bodies, shapes): Self::SystemData) {
        // Synchronize the `Shapes` with `RigidBodies`
        // Contains the entity ID of which need to update the shape information
        let dirty_shapes = {
            let bodies_events = bodies
                .channel()
                .read(self.bodies_event_reader.as_mut().unwrap());
            let shapes_events = shapes
                .channel()
                .read(self.shapes_event_reader.as_mut().unwrap());

            let mut dirty_shapes =
                BitSet::with_capacity((bodies_events.len() + shapes_events.len()) as u32);

            let event_storages = vec![bodies_events, shapes_events];
            event_storages.into_iter().flatten().for_each(|e| match e {
                ComponentEvent::Inserted(index)
                | ComponentEvent::Modified(index)
                | ComponentEvent::Removed(index) => {
                    dirty_shapes.add(*index);
                }
            });

            dirty_shapes
        };

        // Insert or Update shape to `RigidBody`
        for (body, shape, _) in (&bodies, &shapes, &dirty_shapes).join() {
            body_server.0.set_shape(body.get(), Some(shape.get()));
        }

        // Remove shape to `RigidBody`
        for (body, _, _) in (&bodies, !&shapes, &dirty_shapes).join() {
            body_server.0.set_shape(body.get(), None);
        }
    }

    fn setup(&mut self, res: &mut World) {
        {
            let mut storage: WriteStorage<PhysicsHandle<PhysicsBodyTag>> = SystemData::fetch(&res);
            self.bodies_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<PhysicsHandle<PhysicsShapeTag>> = SystemData::fetch(&res);
            self.shapes_event_reader = Some(storage.register_reader());
        }
    }
}