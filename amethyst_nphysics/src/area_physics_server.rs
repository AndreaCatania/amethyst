use amethyst_core::ecs::Entity;
use amethyst_phythyst::{
    objects::*,
    servers::{AreaDesc, AreaPhysicsServerTrait, OverlapEvent},
    PtReal,
};
use nalgebra::Isometry3;
use nphysics3d::{
    object::{
        BodyStatus as NpBodyStatus, Collider as NpCollider, ColliderDesc as NpColliderDesc, ColliderHandle as NpColliderHandle, BodyPartHandle as NpBodyPartHandle,
    },
};

use crate::{rigid_body::RigidBody, conversors::*, servers_storage::*};

pub struct AreaNpServer<N: PtReal> {
    storages: ServersStorageType<N>,
}

impl<N: PtReal> AreaNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        AreaNpServer { storages }
    }
}

// This is a collection of functions that can be used by other servers to perform some common
// operation on the areas.
impl<N: PtReal> AreaNpServer<N> {
    pub fn drop_area(
        area_tag: PhysicsAreaTag,
        areas_storage: &mut AreaStorageWrite,
        colliders_storage: &mut ColliderStorageWrite<N>,
        shapes_storage: &mut ShapeStorageWrite<N>,
    ) {
        let area_key = tag_to_store_key(area_tag.0);
        if let Some(area) = areas_storage.get_mut(area_key) {
            Self::remove_shape(area.as_mut(), shapes_storage, colliders_storage);
        }
        areas_storage.destroy(area_key);
    }

    /// Remove shape.
    /// Take care to unregister the shape and then drop the internal collider.
    pub fn remove_shape(area: &mut Area, shapes: &mut ShapeStorageWrite<N>, colliders: &mut ColliderStorageWrite<N>) {
        if let Some(shape) = shapes.get_mut(area.shape_key) {
            shape.unregister_area(area.self_key.unwrap());
        }
        Self::drop_collider(area, colliders);
    }

    /// Just drop the internal collider of the passed area.
    pub fn drop_collider(area: &mut Area, colliders: &mut ColliderStorageWrite<N>) {
        if let Some(collider_key) = area.collider_key {
            colliders.drop_collider(collider_key);
            area.collider_key = None;
        }
    }

    //pub fn copy_collider_desc(
    //    np_collider: &mut NpCollider<N>,
    //    collider_desc: &mut NpColliderDesc<N>,
    //) {
    //    collider_desc
    //        .set_is_sensor(true)
    //        .set_position(*np_collider.position());
    //}

    //pub fn install_collider<'w>(
    //    area: &mut Area,
    //    collider_desc: &NpColliderDesc<N>,
    //    colliders: &mut ColliderStorageWrite<N>,
    //) {
    //    let collider = collider_desc.build(NpBodyPartHandle());
    //    AreaNpServer::update_user_data(collider, area);

    //    // Collider registration
    //    area.collider_handle = Some(collider.handle());
    //}

    //pub fn update_user_data(collider: &mut NpCollider<N>, area: &Area) {
    //    collider.set_user_data(Some(Box::new(UserData::new(
    //        ObjectType::Area,
    //        *area.self_tag.unwrap(),
    //        area.entity,
    //    ))));
    //}
}

impl<N> AreaPhysicsServerTrait for AreaNpServer<N>
where
    N: PtReal,
{
    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsHandle<PhysicsAreaTag> {
        // TODO later I want to split these in two different APIs, so I'm developing them already separated.

        let ph = {
            let mut bodies_storage = self.storages.rbodies_w();
            let mut colliders = self.storages.colliders_w();
            let mut shape_storage = self.storages.shapes_w();

            // Create Rigid body
            let np_rigid_body = NpRigidBodyDesc::new()
                .set_status(NpBodyStatus::Static)
                .set_mass(N::from(0.0f32))
                .build();

            let a_key = bodies_storage.insert_body(Box::new(RigidBody::new_area(Box::new(np_rigid_body), tag_to_store_key(world_tag.0))));
            let area = bodies_storage.get_body_mut(a_key).unwrap();
            area.self_key = Some(b_key);

            PhysicsAreaTag(store_key_to_tag(b_key))
        };

        {
            let body_key = tag_to_store_key(ph.0);
            let mut bodies = self.storages.rbodies_w();

            let shape_key = tag_to_store_key(area_desc.shape);
            if let Some(body) = bodies.get_body_mut(body_key) {
                if body.shape_key != shape_key {
                    let mut colliders = self.storages.colliders_w();
                    let mut shapes = self.storages.shapes_w();

                    // Remove the old shape
                    if let Some(b_shape_key) = body.shape_key {
                        RBodyNpServer::remove_shape(body, &mut shapes, &mut colliders);
                    }

                    // Assign the new shape if shape_tag is Some
                    if let Some(shape_key) = shape_key {
                        if let Some(shape) = shapes.get_mut(shape_key) {

                            // Create and attach the collider
                            let mut collider_desc =
                                NpColliderDesc::new(shape.shape_handle().clone()).density(nalgebra::convert(1.0));

                            RBodyNpServer::install_shape(body, shape, &collider_desc, &mut colliders);
                        }else{
                            error!("During the rigid body creation, was not possible to find the shape to assign");
                        }
                    }
                }
            } else{
                error!("Body not found");
            }
        }

        PhysicsHandle::new(ph, self.storages.gc.clone())
    }

    fn set_entity(&self, area_tag: PhysicsAreaTag, entity: Option<Entity>) {
        let mut area_storage = self.storages.areas_w();
        let area = storage_safe_get_mut!(area_storage, area_tag);
        area.entity = entity;

        if area.collider_handle.is_none() {
            return;
        }
        let mut world_storage = self.storages.worlds_w();
        let world = storage_safe_get_mut!(world_storage, area.world_tag);
        let collider = world.collider_mut(area.collider_handle.unwrap()).unwrap();

        AreaNpServer::update_user_data(collider, area);
    }

    fn entity(&self, area_tag: PhysicsAreaTag) -> Option<Entity> {
        let area_storage = self.storages.areas_r();
        let area = storage_safe_get!(area_storage, area_tag, None);
        area.entity
    }

    fn set_body_transform(&self, area_tag: PhysicsAreaTag, transf: &Isometry3<f32>) {
        let mut areas_storage = self.storages.areas_w();
        let mut worlds_storage = self.storages.worlds_w();

        let area = storage_safe_get!(areas_storage, area_tag);
        let world = storage_safe_get_mut!(worlds_storage, area.world_tag);

        let transf = TransfConversor::to_physics(transf);

        {
            // TODO remove this if the actual NPHysics got updated since actualy there' a bug (v0.11.1)
            world
                .collider_world_mut()
                .set_position(area.collider_handle.unwrap(), transf.clone());
        }

        // Set the position of the collider, this is necessary for static objects
        let np_collider = world.collider_mut(area.collider_handle.unwrap());
        fail_cond!(np_collider.is_none());
        let np_collider = np_collider.unwrap();
        np_collider.set_position(transf);
    }

    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent> {
        let s = self.storages.areas_r();
        let area = s.get(*area_tag);
        fail_cond!(area.is_none(), Vec::new());
        area.unwrap().overlap_events.to_vec()
    }
}
