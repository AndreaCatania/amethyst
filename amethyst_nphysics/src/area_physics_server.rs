use amethyst_core::ecs::Entity;
use amethyst_phythyst::{
    objects::*,
    servers::{AreaDesc, AreaPhysicsServerTrait, OverlapEvent},
    PtReal,
};
use log::error;
use nalgebra::Isometry3;
use nphysics3d::object::{
    BodyPartHandle as NpBodyPartHandle, BodyStatus as NpBodyStatus, Collider as NpCollider,
    ColliderDesc as NpColliderDesc, ColliderHandle as NpColliderHandle, RigidBody as NpRigidBody,
    RigidBodyDesc as NpRigidBodyDesc,
};

use crate::{
    conditional_macros,
    conversors::*,
    body::{BodyData, Body},
    servers_storage::*,
    shape::RigidShape,
    storage::StoreKey,
    utils::*,
};

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
        bodies_storage: &mut BodiesStorageWrite<N>,
        colliders_storage: &mut CollidersStorageWrite<N>,
        shapes_storage: &mut ShapesStorageWrite<N>,
    ) {
        let area_key = tag_to_store_key(area_tag.0);
        if let Some(area) = bodies_storage.get_body_mut(area_key) {
            Self::remove_shape(area, shapes_storage, colliders_storage);
        }
        bodies_storage.drop_body(area_key);
    }

    /// Set shape.
    /// Take care to register the shape and set the collider to the body.
    pub fn install_shape<'w>(
        area: &mut Body<N>,
        shape: &mut RigidShape<N>,
        collider_desc: &NpColliderDesc<N>,
        colliders: &mut CollidersStorageWrite<N>,
    ) {
        Self::install_collider(area, collider_desc, colliders);

        // Collider registration
        shape.register_body(area.self_key.unwrap());
        area.shape_key = shape.self_key;
    }

    /// Remove shape.
    /// Take care to unregister the shape and then drop the internal collider.
    pub fn remove_shape(
        area: &mut Body<N>,
        shapes: &mut ShapesStorageWrite<N>,
        colliders: &mut CollidersStorageWrite<N>,
    ) {
        if let Some(shape_key) = area.shape_key {
            if let Some(shape) = shapes.get_mut(shape_key) {
                shape.unregister_body(area.self_key.unwrap());
            } else {
                error!("An area is associated with a shape, but the shape doesn't exist!");
            }
            area.shape_key = None;
        }
        Self::drop_collider(area, colliders);
    }

    pub fn install_collider<'w>(
        area: &mut Body<N>,
        collider_desc: &NpColliderDesc<N>,
        colliders: &mut CollidersStorageWrite<N>,
    ) {
        let mut collider = collider_desc.build(NpBodyPartHandle(area.self_key.unwrap(), 0));
        AreaNpServer::update_user_data(&mut collider, area);

        let key = colliders.insert_collider(Box::new(collider));
        area.collider_key = Some(key);
    }

    /// Just drop the internal collider of the passed area.
    pub fn drop_collider(area: &mut Body<N>, colliders: &mut CollidersStorageWrite<N>) {
        if let Some(collider_key) = area.collider_key {
            colliders.drop_collider(collider_key);
            area.collider_key = None;
        }
    }

    pub fn update_user_data(collider: &mut NpCollider<N, StoreKey>, area: &Body<N>) {
        collider.set_user_data(Some(Box::new(UserData::new(
            ObjectType::Area,
            area.self_key.unwrap(),
            area.entity,
        ))));
    }

    pub fn extract_collider_desc(
        np_rigid_body: &NpRigidBody<N>,
        collider_desc: &mut NpColliderDesc<N>,
    ) {
        collider_desc
            .set_is_sensor(true);
    }
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
            let mut bodies_storage = self.storages.bodies_w();
            let mut colliders = self.storages.colliders_w();
            let mut shape_storage = self.storages.shapes_w();

            // Create Rigid body
            let np_rigid_body = NpRigidBodyDesc::new()
                .set_status(NpBodyStatus::Static)
                .set_mass(N::from(0.0f32))
                .build();

            let a_key = bodies_storage.insert_body(Box::new(Body::new_area(
                Box::new(np_rigid_body),
                tag_to_store_key(world_tag.0),
            )));
            let area = bodies_storage.get_body_mut(a_key).unwrap();
            area.self_key = Some(a_key);

            PhysicsAreaTag(store_key_to_tag(a_key))
        };

        {
            let body_key = tag_to_store_key(ph.0);
            let mut bodies = self.storages.bodies_w();

            let shape_key = Option::Some(tag_to_store_key(area_desc.shape.0));
            if let Some(body) = bodies.get_body_mut(body_key) {
                if body.shape_key != shape_key {
                    let mut colliders = self.storages.colliders_w();
                    let mut shapes = self.storages.shapes_w();

                    // Remove the old shape
                    if let Some(b_shape_key) = body.shape_key {
                        AreaNpServer::remove_shape(body, &mut shapes, &mut colliders);
                    }

                    // Assign the new shape if shape_tag is Some
                    if let Some(shape_key) = shape_key {
                        if let Some(shape) = shapes.get_mut(shape_key) {
                            // Create and attach the collider
                            let mut collider_desc =
                                NpColliderDesc::new(shape.shape_handle().clone()).sensor(true);

                            AreaNpServer::install_shape(
                                body,
                                shape,
                                &collider_desc,
                                &mut colliders,
                            );
                        } else {
                            error!("During the area creation, was not possible to find the shape to assign");
                        }
                    }
                }
            } else {
                error!("Area body not found");
            }
        }

        PhysicsHandle::new(ph, self.storages.gc.clone())
    }

    fn set_entity(&self, area_tag: PhysicsAreaTag, entity: Option<Entity>) {
        let area_key = tag_to_store_key(area_tag.0);
        let mut bodies = self.storages.bodies_w();

        if let Some(area) = bodies.get_body_mut(area_key) {
            fail_cond!(!matches!(area.body_data, BodyData::Area(_)));
            area.entity = entity;

            if let Some(collider_key) = area.collider_key {
                let mut colliders = self.storages.colliders_w();
                if let Some(collider) = colliders.get_collider_mut(collider_key) {
                    AreaNpServer::update_user_data(collider, area);
                } else {
                    error!("A body is assigned to a collider, but the collider doesn't exist!")
                }
            }
        }
    }

    fn entity(&self, area_tag: PhysicsAreaTag) -> Option<Entity> {
        let area_key = tag_to_store_key(area_tag.0);
        let mut bodies = self.storages.bodies_r();

        if let Some(area) = bodies.get_body(area_key) {
            area.entity
        } else {
            None
        }
    }

    fn set_body_transform(&self, area_tag: PhysicsAreaTag, transf: &Isometry3<f32>) {
        let area_key = tag_to_store_key(area_tag.0);
        let mut bodies = self.storages.bodies_w();

        if let Some(area) = bodies.get_body_mut(area_key) {
            if let Some(body) = area.rigid_body_mut() {
                body.set_position(TransfConversor::to_physics(transf));
            } else {
                error!("Failed to cast the body, to a Rigid Body!");
            }
        }
    }

    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent> {
        let area_key = tag_to_store_key(area_tag.0);
        let s = self.storages.bodies_r();
        if let Some(area) = s.get_body(area_key) {
            if let BodyData::Area(e) = &area.body_data {
                return e.to_vec();
            }
        }
        Vec::new()
    }
}
