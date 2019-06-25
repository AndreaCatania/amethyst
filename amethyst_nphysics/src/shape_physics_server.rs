use amethyst_phythyst::{
    servers::ShapePhysicsServerTrait,
};
use crate::{
    servers_storage::*,
    conversors::*,
};

pub struct ShapeNpServer{
    storages: ServersStorageType,
}

impl ShapePhysicsServerTrait for ShapeNpServer{
    fn create_shape(&mut self){

    }

    fn drop_shape(&mut self){

    }

    fn update_shape(&mut self){

    }
}