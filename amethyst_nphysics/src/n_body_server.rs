
use amethyst_phythyst::{
    servers::{
        PhysicsWorldTag,
        PhysicsBodyTag,
        RigidBodyServer,
    },
};

use crate::{
    storage::Storage,
    rigid_body::ARigidBody
};

pub struct NRigidBodyServer {
    storage: Storage<Box<ARigidBody>>,
}

impl NRigidBodyServer {

    pub fn new() -> Self{
        NRigidBodyServer {
            storage: Storage::new(50, 50),
        }
    }
}

impl RigidBodyServer for NRigidBodyServer {

    fn create(&mut self) -> PhysicsBodyTag {
        let body = ARigidBody::new();
        PhysicsBodyTag(self.storage.make_opaque(body))
    }

    fn drop(&mut self, body: PhysicsBodyTag){
        self.storage.drop(body.0);
    }
}