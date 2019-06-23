
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
        PhysicsBodyTag(self.storages.rbodies_w().make_opaque(body))
    }

    fn drop_body(&mut self, body: PhysicsBodyTag){
        self.storages.rbodies_w().destroy(*body);
    }
}