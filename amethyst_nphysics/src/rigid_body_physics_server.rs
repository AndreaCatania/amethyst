
use crate::{
    servers_storage::*,
    rigid_body::RigidBody
};

use amethyst_phythyst::{
    servers::{
        RBodyPhysicsServerTrait,
        RigidBodyDesc,
    },
    objects::*,
};

use amethyst_core::{
    components::Transform,
    Float,
    math::{
        Vector3,
        Translation3,
        Quaternion,
        Vector4,
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

use ncollide3d::{
    shape::{
        ShapeHandle as NcShapeHandle,
        Ball as NcBall,
    }
};

use nalgebra::Isometry3;

pub struct RBodyNpServer {
    storages: ServersStorageType,
}

macro_rules! extract_rigid_body {
    ($_self:ident, $body:ident) => {

        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = bodies_storage.get(*$body);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();

        let $body = ServersStorage::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();

    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {

        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = bodies_storage.get(*$body);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();

        let $body = ServersStorage::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();

    }
}

impl RBodyNpServer {

    pub fn new(storages: ServersStorageType) -> Self{
        RBodyNpServer {
            storages
        }
    }

}

impl RBodyPhysicsServerTrait for RBodyNpServer {

    fn create_body(&mut self, world_tag: PhysicsWorldTag, body_desc : &RigidBodyDesc) -> PhysicsBodyTag {

        let mut world_storage = self.storages.worlds_w();

        let world = world_storage.get_mut(*world_tag);
        assert!(world.is_some());
        let world = world.unwrap();

        let v : &Vector3<Float> = body_desc.transformation.translation();
        let r = body_desc.transformation.rotation();
        let pos = Isometry3::from_parts(Translation3::from(Vector3::<f32>::new(v.x.as_f32(), v.y.as_f32(), v.z.as_f32())),UnitQuaternion::new_normalize(Quaternion::from(Vector4::new(r.i.as_f32(), r.j.as_f32(), r.k.as_f32(), r.w.as_f32()))) );

        let mut collider_desc = NpColliderDesc::new(NcShapeHandle::new(NcBall::new(0.01)) )
            .density(body_desc.mass);

        let rb = NpRigidBodyDesc::new()
            .collider(&collider_desc)
            .set_position(pos)
            .build(world);

        PhysicsBodyTag(self.storages.rbodies_w().make_opaque(RigidBody::new(rb.handle(), world_tag)))
    }

    fn drop_body(&mut self, body: PhysicsBodyTag){
        unimplemented!();
        self.storages.rbodies_w().destroy(*body);
    }

    fn body_transform(&self, body: PhysicsBodyTag) -> Transform {

        extract_rigid_body!(self, body, Transform::default());

        let t: &Isometry3<f32> = body.position();
        Transform::new(t.translation, t.rotation, Vector3::new(1.0, 1.0, 1.0))
    }
}