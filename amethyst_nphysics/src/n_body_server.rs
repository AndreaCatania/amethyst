
use amethyst_phythyst::{
    servers::{
        PhysicsWorldTag,
        PhysicsBodyTag,
        RigidBodyServer,
    },
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

impl RigidBodyServer for NRigidBodyServer {

    fn create(&mut self) -> PhysicsBodyTag {
        let body = ARigidBody::new();
        PhysicsBodyTag(self.storages.write().unwrap().rigid_bodies.make_opaque(body))
    }

    fn drop(&mut self, body: PhysicsBodyTag){
        self.storages.write().unwrap().rigid_bodies.drop(body.0);
    }
}