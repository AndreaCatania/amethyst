use crate::{objects::*, servers::*};
use amethyst_core::{
    ecs::{storage::ComponentEvent, Entities, ReaderId, BitSet, SystemData, Join, ReadExpect, ReadStorage, System, WriteStorage, Resources,},
    transform::components::{
        Transform,
        Parent,
    },
    math::{
        Isometry3,
        Quaternion,
    },
    Float,
};

pub struct PhysicsSyncTransformSystem {
    transf_event_reader: Option<ReaderId<ComponentEvent>>,
    rigid_bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    areas_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl PhysicsSyncTransformSystem {
    pub fn new() -> PhysicsSyncTransformSystem {
        PhysicsSyncTransformSystem {
            transf_event_reader: None,
            rigid_bodies_event_reader: None,
            areas_event_reader: None,
        }
    }

    fn compute_transform(parent: &Parent, transforms: &WriteStorage<Transform>, parents: &ReadStorage<Parent>) -> Isometry3<Float> {
        let i = transforms.get(parent.entity).map_or(
            Isometry3::identity(),
            |t| t.isometry().clone());

        if let Some(parent_parent) = parents.get(parent.entity) {
            i * Self::compute_transform(parent_parent, transforms, parents)
        } else {
            i
        }
    }

    fn setup_step_2(&mut self, res: &Resources) {

        {
            let mut storage: WriteStorage<Transform> = SystemData::fetch(&res);
            self.transf_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<PhysicsHandle<PhysicsBodyTag>> = SystemData::fetch(&res);
            self.rigid_bodies_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<PhysicsHandle<PhysicsAreaTag>> = SystemData::fetch(&res);
            self.areas_event_reader = Some(storage.register_reader());
        }
    }
}

impl<'a> System<'a> for PhysicsSyncTransformSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, RBodyPhysicsServer<f32>>,
        ReadExpect<'a, AreaPhysicsServer>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
        ReadStorage<'a, Parent>,
    );

    define_setup_with_physics_assertion!(setup_step_2);

    fn run(&mut self, (entities, rbody_server, area_server, mut transforms, bodies, areas, parents): Self::SystemData) {

        let mut edited_transforms = BitSet::new();

        // Collect all information about the entities that want to update the transform
        {
            let events = transforms.channel().read(self.transf_event_reader.as_mut().unwrap());
            for e in events {
                match e {
                    // TODO
                    // Removing the below comment allow to fully synchronize the transform
                    // This mean that changing a transform result in an automatic update of the object
                    // The problem with this is that due to this issue is not yet possible do it:
                    // https://github.com/amethyst/amethyst/issues/1795
                    //
                    ComponentEvent::Inserted(index) /*| ComponentEvent::Modified(index)*/ => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
        }
        {
            let events = bodies.channel().read(self.rigid_bodies_event_reader.as_mut().unwrap());
            for e in events {
                match e {
                    ComponentEvent::Inserted(index) => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
        }
        {
            let events = areas.channel().read(self.areas_event_reader.as_mut().unwrap());
            for e in events {
                match e {
                    ComponentEvent::Inserted(index) => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
        }

        // Set transform to physics with no parents

        for (transform, rb_tag, _, _,) in (&transforms, &bodies, !&parents, &edited_transforms).join() {
            rbody_server.set_body_transform(rb_tag.get(), transform);
        }

        for (transform, a_tag, _, _,) in (&transforms, &areas, !&parents, &edited_transforms).join() {
            area_server.set_body_transform(a_tag.get(), transform);
        }

        // Set transform to physics with parents

        for (transform, a_tag, parent, _,) in (&transforms, &areas, &parents, &edited_transforms).join() {

            let computed_trs = transform.isometry() * Self::compute_transform(parent, &transforms, &parents);
            let mut t = Transform::default();
            t.set_isometry(computed_trs);
            area_server.set_body_transform(a_tag.get(), &t);
        }

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

