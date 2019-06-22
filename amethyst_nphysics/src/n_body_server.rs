
use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
    },
    objects::*,
};

use crate::{
    servers_storage::{ServersStorageType},
    rigid_body::ARigidBody
};

pub struct NRigidBodyServer {
    storages: ServersStorageType,
}

impl NRigidBodyServer {

    pub fn new(storages: ServersStorageType) -> Self{
        NRigidBodyServer {
            storages
        }
    }
}

impl RBodyPhysicsServerTrait for NRigidBodyServer {

    fn create_body(&mut self) -> PhysicsBodyTag {
        let body = ARigidBody::new();
        PhysicsBodyTag(storage_write!(self).rigid_bodies.make_opaque(body))
    }

    fn drop_body(&mut self, body: PhysicsBodyTag){
        storage_write!(self).rigid_bodies.drop(body.0);
    }
}