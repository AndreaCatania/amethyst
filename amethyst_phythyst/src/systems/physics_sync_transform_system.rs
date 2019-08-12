use crate::{objects::*, servers::*};
use amethyst_core::{
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Join, ReadExpect, ReadStorage, ReaderId, System,
        SystemData, World, WriteStorage,
    },
    math::{Isometry3, Quaternion, RealField},
    transform::components::{Parent, Transform},
};

pub struct PhysicsSyncTransformSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    transf_event_reader: Option<ReaderId<ComponentEvent>>,
    rigid_bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    areas_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<N: crate::PtReal> PhysicsSyncTransformSystem<N> {
    pub fn new() -> PhysicsSyncTransformSystem<N> {
        PhysicsSyncTransformSystem {
            phantom_data: std::marker::PhantomData,
            transf_event_reader: None,
            rigid_bodies_event_reader: None,
            areas_event_reader: None,
        }
    }

    /// This method resolve the transformation of an object that is attached to a parent
    /// TODO each time an object of this type receive an edit this compute the entire chain
    /// of parents. Is mandatory find a way to optimize this process.
    /// Is it possible to directly use Global matrix?
    fn compute_transform(
        parent: &Parent,
        transforms: &WriteStorage<Transform>,
        parents: &ReadStorage<Parent>,
    ) -> Isometry3<f32> {
        let i = transforms
            .get(parent.entity)
            .map_or(Isometry3::identity(), |t| t.isometry().clone());

        if let Some(parent_parent) = parents.get(parent.entity) {
            i * Self::compute_transform(parent_parent, transforms, parents)
        } else {
            i
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsSyncTransformSystem<N> {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, RBodyPhysicsServer<N>>,
        ReadExpect<'a, AreaPhysicsServer>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsHandle<PhysicsBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
        ReadStorage<'a, Parent>,
    );

    fn run(
        &mut self,
        (entities, rbody_server, area_server, mut transforms, bodies, areas, parents): Self::SystemData,
    ) {
        let mut edited_transforms;
        {
            let trs_events = transforms
                .channel()
                .read(self.transf_event_reader.as_mut().unwrap());
            let bodies_events = bodies
                .channel()
                .read(self.rigid_bodies_event_reader.as_mut().unwrap());
            let area_events = areas
                .channel()
                .read(self.areas_event_reader.as_mut().unwrap());

            edited_transforms = BitSet::with_capacity(
                (trs_events.len() + bodies_events.len() + area_events.len()) as u32,
            );

            // Collect all information about the entities that want to update the transform
            for e in trs_events {
                match e {
                    // TODO
                    // Removing the below comment allow to fully synchronize the transform
                    // This mean that changing a transform result in an automatic update of the object
                    // The problem with this is that due to this issue is not yet possible do it:
                    // https://github.com/amethyst/amethyst/issues/1795
                    //
                    ComponentEvent::Inserted(index) | ComponentEvent::Modified(index) => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
            for e in bodies_events {
                match e {
                    ComponentEvent::Inserted(index) => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
            for e in area_events {
                match e {
                    ComponentEvent::Inserted(index) => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
        }

        // Set transform to physics with no parents

        for (transform, rb_tag, _, _) in
            (&transforms, &bodies, !&parents, &edited_transforms).join()
        {
            rbody_server.set_body_transform(rb_tag.get(), transform.isometry());
        }

        for (transform, a_tag, _, _) in (&transforms, &areas, !&parents, &edited_transforms).join()
        {
            area_server.set_body_transform(a_tag.get(), transform.isometry());
        }

        // Set transform to physics with parents

        for (transform, rb_tag, parent, _) in
            (&transforms, &bodies, &parents, &edited_transforms).join()
        {
            let computed_trs =
                transform.isometry() * Self::compute_transform(parent, &transforms, &parents);
            rbody_server.set_body_transform(rb_tag.get(), &computed_trs);
        }

        for (transform, a_tag, parent, _) in
            (&transforms, &areas, &parents, &edited_transforms).join()
        {
            let computed_trs =
                transform.isometry() * Self::compute_transform(parent, &transforms, &parents);
            area_server.set_body_transform(a_tag.get(), &computed_trs);
        }

        // Sync transform back to Amethyst.
        // Note that the transformation are modified in this way to avoid to mutate the
        // Transform component entirely.
        // TODO find a way to update only moving things and not always all
        let transf_mask = transforms.mask().clone();
        for (entity, rb, _) in (&entities, &bodies, &transf_mask & !&edited_transforms).join() {
            match transforms.get_mut(entity) {
                Some(transform) => {
                    // TODO please avoid much copies by sending the mutable reference directly
                    transform.set_isometry(rbody_server.body_transform(rb.get()));
                }
                _ => {}
            }
        }

        // TODO update Area is missing here

        // Now the transformation get changed by the synchronization and we don't need such events,
        // So consume them now.
        transforms
            .channel()
            .read(self.transf_event_reader.as_mut().unwrap());
    }

    fn setup(&mut self, res: &mut World) {
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
