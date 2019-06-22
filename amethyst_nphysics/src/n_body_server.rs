
use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
    },
    objects::*,
};

use crate::{
    storages::Storages,
    rigid_body::ARigidBody
};

use std::sync::{
    Arc, RwLock,
};

pub struct NRigidBodyServer {
    storages: Arc<RwLock<Storages>>,
}

impl NRigidBodyServer {

    pub fn new(storages: Arc<RwLock<Storages>>) -> Self{
        NRigidBodyServer {
            storages
        }
    }
}

impl RBodyPhysicsServerTrait for NRigidBodyServer {

    fn create_body(&mut self) -> PhysicsBodyTag {
        let body = ARigidBody::new();
        PhysicsBodyTag(self.storages.write().unwrap().rigid_bodies.make_opaque(body))
    }

    fn drop_body(&mut self, body: PhysicsBodyTag){
        self.storages.write().unwrap().rigid_bodies.drop(body.0);
    }
}