
use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
    },
    objects::*,
};

use crate::{
    servers_storage::{ServersStorageType},
    rigid_body::RigidBody
};

pub struct RBodyNpServer {
    storages: ServersStorageType,
}

impl RBodyNpServer {

    pub fn new(storages: ServersStorageType) -> Self{
        RBodyNpServer {
            storages
        }
    }
}

impl RBodyPhysicsServerTrait for RBodyNpServer {

    fn create_body(&mut self) -> PhysicsBodyTag {
        let body = RigidBody::new();
        PhysicsBodyTag(self.storages.rbodies_w().make_opaque(body))
    }

    fn drop_body(&mut self, body: PhysicsBodyTag){
        self.storages.rbodies_w().destroy(*body);
    }
}