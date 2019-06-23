
use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
    },
    objects::*,
};
use amethyst_core::components::Transform;
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

    fn body_transform(&self, body: PhysicsBodyTag) -> Transform {

        let body = self.storages.rbodies_r().get(PhysicsBodyTag);
        fail_cond!(body.is_none());
        let body = body.unwrap();

        body.get_handle();

        Transform::default()
    }
}