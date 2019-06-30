
use crate::servers_storage::*;
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

impl<N: RealField> AreaPhysicsServerTrait for AreaNpServer<N> {

    fn create_area(
        &mut self,
        world_tag: PhysicsWorldTag,
        area_desc: &AreaDesc,
    ) -> PhysicsAreaTag {
        let mut worlds_storage = self.storages.worlds_w();
        // area storage ?
        let mut shapes_storage = self.storages.shapes_w();

        let np_world = worlds_storage.get_mut(*world_tag).expect("During the area creation the world tag passed was not valid");
        let shape = shapes_storage.get_mut(*area_desc.shape).expect("During area creation was not possible to find the shape");

        let np_collider = NpColliderDesc::new(shape.shape_handle().clone()).sensor(true).build(np_world);

        PhysicsAreaTag::default()
    }

    fn drop_area(&mut self, area_tag: PhysicsAreaTag){
        unimplemented!();
    }
}
