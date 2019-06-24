
use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
    },
    objects::*,
};
use amethyst_core::{
    components::Transform,
    Float,
    math::{
        Vector3,
        Translation3,
        UnitQuaternion,
    },
};
use nalgebra::Isometry3;
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

    fn body_transform(&self, world: PhysicsWorldTag, body: PhysicsBodyTag) -> Transform {

        // TODO Improve this function it has too much overhead.

        let w = self.storages.worlds_r();

        let world = w.get(*world);
        fail_cond!(world.is_none(), Transform::default());
        let world = world.unwrap();

        let r = self.storages.rbodies_r();

        let body = r.get(*body);
        fail_cond!(body.is_none(), Transform::default());
        let body = body.unwrap();

        let body = body.get_handle();
        fail_cond!(body.is_none(), Transform::default());
        let body = world.rigid_body(body.unwrap()).unwrap();

        let t: &Isometry3<f32> = body.position();

        let position = t.translation;
        let rotation = t.rotation;
        let scale = Vector3::new(1.0, 1.0, 1.0);

        let t = Transform::new(position, rotation, scale);

        dbg!(position);

        t
    }
}