use crate::{area::Area, conversors::*, servers_storage::*};
use amethyst_core::ecs::Entity;
use amethyst_phythyst::{
    objects::*,
    servers::{AreaDesc, AreaPhysicsServerTrait, OverlapEvent},
    PtReal,
};
use nalgebra::Isometry3;
use nphysics3d::{
    object::{
        Collider as NpCollider, ColliderDesc as NpColliderDesc, ColliderHandle as NpColliderHandle,
    },
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
// operation on areas.
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

    //pub fn set_collider<'w>(
    //    area: &mut Area,
    //    area_tag: PhysicsAreaTag,
    //    np_world: &'w mut NpWorld<N>,
    //    collider_desc: &NpColliderDesc<N>,
    //) {
    //    let collider = collider_desc.build(np_world);
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

/*
impl<N> AreaPhysicsServerTrait for AreaNpServer<N>
where
    N: PtReal,
{
    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsHandle<PhysicsAreaTag> {
        let mut worlds_storage = self.storages.worlds_w();
        let mut areas_storage = self.storages.areas_w();
        let mut shapes_storage = self.storages.shapes_w();

        let np_world = worlds_storage
            .get_mut(*world_tag)
            .expect("During the area creation the world tag passed was not valid");
        let shape = shapes_storage
            .get_mut(*area_desc.shape)
            .expect("During area creation was not possible to find the shape");

        let area_tag = PhysicsAreaTag(
            areas_storage.make_opaque(Box::new(Area::new(world_tag, area_desc.shape))),
        );
        let area = areas_storage.get_mut(*area_tag).unwrap();
        area.self_tag = Some(area_tag);

        shape.register_area(area_tag);

        let np_collider_desc = NpColliderDesc::new(shape.shape_handle().clone()).sensor(true);

        AreaNpServer::set_collider(area, area_tag, np_world, &np_collider_desc);

        PhysicsHandle::new(area_tag, self.storages.gc.clone())
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
*/