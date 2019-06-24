
use crate::{
    servers_storage::*,
    rigid_body::RigidBody
};

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

use nphysics3d::{
    object::{
        RigidBody as NpRigidBody,
        RigidBodyDesc as NpRigidBodyDesc,
        ColliderDesc as NpColliderDesc,
        BodyHandle as NpBodyHandle,
        BodyStatus as NpBodyStatus,
    },
};

use nalgebra::Isometry3;



pub struct RBodyNpServer {
    storages: ServersStorageType,
}

macro_rules! extract_rigid_body {
    ($_self:ident, $body:ident) => {

        let worlds_storage = $_self.storages.worlds_r();
        let $body = {
            let (__body, __world) = $body.unwrap().get_handle().unwrap();

            let world = worlds_storage.get(*__world);
            fail_cond!(world.is_none());
            let __body = world.unwrap().rigid_body(__body);
            fail_cond!(__body.is_none());
            __body
        }
    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {

        let worlds_storage = $_self.storages.worlds_r();
        let $body = {
            let (__body, __world) = $body.unwrap().get_handle().unwrap();

            let world = worlds_storage.get(*__world);
            fail_cond!(world.is_none(), $on_fail_ret);
            let __body = world.unwrap().rigid_body(__body);
            fail_cond!(__body.is_none(), $on_fail_ret);
            __body.unwrap()
        }
    }
}

impl RBodyNpServer {

    pub fn new(storages: ServersStorageType) -> Self{
        RBodyNpServer {
            storages
        }
    }

//    pub fn rigid_body<'s>(storage :&'s WorldStorageRead, world_tag: PhysicsWorldTag, body_handle: NpBodyHandle) -> Option<&'s NpRigidBody<f32>> {
//
//        let world = storage.get(*world_tag);
//        fail_cond!(world.is_none(), None);
//
//        world.unwrap().rigid_body(body_handle)
//    }

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

        let bodies_storage = self.storages.rbodies_r();

        let body = bodies_storage.get(*body);
        fail_cond!(body.is_none(), Transform::default());

        extract_rigid_body!(self, body, Transform::default());

        let t: &Isometry3<f32> = body.position();

        let position = t.translation;
        let rotation = t.rotation;
        let scale = Vector3::new(1.0, 1.0, 1.0);

        let t = Transform::new(position, rotation, scale);

        dbg!(position);

        t
    }
}