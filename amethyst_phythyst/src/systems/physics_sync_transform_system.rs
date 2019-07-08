use crate::{objects::*, servers::RBodyPhysicsServer};
use amethyst_core::{
    ecs::{storage::ComponentEvent, Entities, ReaderId, BitSet, SystemData, Join, ReadExpect, ReadStorage, System, WriteStorage, Resources,},
    transform::components::Transform,
};

pub struct PhysicsSyncTransformSystem {
    transf_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl PhysicsSyncTransformSystem {
    pub fn new() -> PhysicsSyncTransformSystem {
        PhysicsSyncTransformSystem { transf_event_reader: None, }
    }

    fn setup_step_2(&mut self, res: &Resources) {


        let mut transform_storage: WriteStorage<Transform> = SystemData::fetch(&res);
        self.transf_event_reader = Some(transform_storage.register_reader());
    }
}

impl<'a> System<'a> for PhysicsSyncTransformSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, RBodyPhysicsServer<f32>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
    );

    define_setup_with_physics_assertion!(setup_step_2);

    fn run(&mut self, (entities, rbody_server, mut transforms, bodies): Self::SystemData) {

        // Synchronize all transformation to the physics

        let events = transforms.channel().read(self.transf_event_reader.as_mut().unwrap());

        let mut edited_transforms = BitSet::new();

        for e in events {
            match e {
                ComponentEvent::Inserted(index) /*| ComponentEvent::Modified(index)*/ => {
                    edited_transforms.add(*index);
                }
                _ => {}
            }
        }

        for (transform, rb_tag, _) in (&transforms, &bodies, &edited_transforms).join() {
            rbody_server.set_body_transform(rb_tag.get(), transform);
            println!("Update trasnsform");
        }

        // TODO Update area?

        // Sync transform back to Amethyst.
        // Note that the transformation are modified in this way to avoid to mutate the
        // Transform component entirely.
        let transf_mask = transforms.mask().clone();
        for (entity, rb, _ ) in (&entities, &bodies, &transf_mask & ! &edited_transforms).join() {
            match transforms.get_mut(entity) {
                Some(transform) => {

                    // TODO please avoid much copies by sending the mutable reference directly
                    *transform = rbody_server.body_transform(rb.get());
                }
                _ => {}
            }
        }

        // Now the transformation get changed by the synchronization and we don't need such events,
        // So consume them now.
        transforms.channel().read(self.transf_event_reader.as_mut().unwrap());
    }
}
