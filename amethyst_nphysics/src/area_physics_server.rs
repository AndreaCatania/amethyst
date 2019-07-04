
use crate::{
    utils::*,
    servers_storage::*,
    area::Area,
    conversors::*,
};
use amethyst_phythyst::{
    servers::{
        AreaPhysicsServerTrait,
        AreaDesc,
    },
    objects::*,
};
use nphysics3d::{
    object::{
        Collider as NpCollider,
        ColliderHandle as NpColliderHandle,
        ColliderDesc as NpColliderDesc,
    },
    world::World as NpWorld,
};
use nalgebra::{
    RealField,
};
use amethyst_phythyst::servers::OverlapEvent;

pub struct AreaNpServer<N: RealField>{
    storages: ServersStorageType<N>
}

impl<N: RealField> AreaNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self{
        AreaNpServer{
            storages,
        }
    }
}

// This is a collection of functions that can be used by other servers to perform some common
// operation on areas.
impl<N: RealField> AreaNpServer<N> {

    pub fn drop_body(area_tag: PhysicsAreaTag, worlds_storage: &mut WorldStorageWrite<N>, areas_storage: &mut AreaStorageWrite, shapes_storage: &mut ShapeStorageWrite<N>) {
        {
            let area = storage_safe_get!(areas_storage, area_tag);

            // Remove from world
            let world = storage_safe_get_mut!(worlds_storage, area.world_tag);
            if let Some(handle) = area.collider_handle {
                world.remove_colliders(&[handle]);
            }

            // Remove from shape
            let shape = storage_safe_get_mut!(shapes_storage, area.shape_tag);
            shape.unregister_area(area_tag);
        }

        areas_storage.destroy(*area_tag);
    }

    pub fn destroy_collider(area: &mut Area, world: &mut NpWorld<N>) {
        fail_cond!(area.collider_handle.is_none());
        world.remove_colliders(&[area.collider_handle.unwrap()]);
        area.collider_handle = None;
    }

    pub fn copy_collider_desc(
        np_collider: &mut NpCollider<N>,
        collider_desc: &mut NpColliderDesc<N>,
    ) {
        collider_desc
            .set_is_sensor(true)
            .set_position(*np_collider.position());
    }

    pub fn set_collider<'w>(
        area: &mut Area,
        area_tag: PhysicsAreaTag,
        np_world: &'w mut NpWorld<N>,
        collider_desc: &NpColliderDesc<N>,
    ) {
        let collider = collider_desc.build(np_world);
        collider.set_user_data(Some(Box::new(UserData::new(ObjectType::Area, *area_tag))));

        // Collider registration
        area.collider_handle = Some(collider.handle());
    }
}

impl<N> AreaPhysicsServerTrait for AreaNpServer<N>
where
    N: RealField,
    amethyst_core::Float: std::convert::From<N>,
    amethyst_core::Float: std::convert::Into<N>,
    N: alga::general::SubsetOf<amethyst_core::Float>,
{

    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsHandle<PhysicsAreaTag> {

        let mut worlds_storage = self.storages.worlds_w();
        let mut areas_storage = self.storages.areas_w();
        let mut shapes_storage = self.storages.shapes_w();

        let np_world = worlds_storage.get_mut(*world_tag).expect("During the area creation the world tag passed was not valid");
        let shape = shapes_storage.get_mut(*area_desc.shape).expect("During area creation was not possible to find the shape");

        let area_tag = PhysicsAreaTag(areas_storage.make_opaque(Box::new(Area::new(None, world_tag, area_desc.shape))));
        let area = areas_storage.get_mut(*area_tag).unwrap();

        shape.register_area(area_tag);

        let np_collider_desc = NpColliderDesc::new(shape.shape_handle().clone())
            .sensor(true)
            .position(TransfConversor::to_physics(&area_desc.transform));

        AreaNpServer::set_collider(area, area_tag, np_world, &np_collider_desc);

        PhysicsHandle::new(area_tag, self.storages.gc.clone())
    }

    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent> {
        let s = self.storages.areas_r();
        let area = s.get(*area_tag);
        fail_cond!(area.is_none(), Vec::new());
        area.unwrap().overlap_events.to_vec()
    }

}

