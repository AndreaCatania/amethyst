
use crate::servers_storage::*;
use amethyst_phythyst::{
    servers::{
        AreaPhysicsServerTrait,
        AreaDesc,
    },
    objects::*,
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
        PhysicsAreaTag::default()
    }

    fn drop_area(&mut self, area_tag: PhysicsAreaTag){
        unimplemented!();
    }
}

