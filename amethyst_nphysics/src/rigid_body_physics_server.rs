use crate::{conversors::*, rigid_body::RigidBody, servers_storage::*};

use amethyst_phythyst::{
    objects::*,
    servers::{RBodyPhysicsServerTrait, RigidBodyDesc},
};

use amethyst_core::components::Transform;

use nphysics3d::{
    object::{
        Body as NpBody, BodyHandle as NpBodyHandle, BodyPartHandle as NpBodyPartHandle,
        BodyStatus as NpBodyStatus, ColliderDesc as NpColliderDesc, RigidBody as NpRigidBody,
        RigidBodyDesc as NpRigidBodyDesc,
    },
    math::{
        Force, ForceType,
    },
    world::World as NpWorld,
};

use ncollide3d::shape::{Ball as NcBall, ShapeHandle as NcShapeHandle};

use nalgebra::{
    RealField,
    Vector3,
    Point,
};

pub struct RBodyNpServer<N: RealField> {
    storages: ServersStorageType<N>,
}

macro_rules! extract_rigid_body {
    ($_self:ident, $body:ident) => {
        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = bodies_storage.get(*$body);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();

        let $body =
            ServersStorage::<N>::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();
    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {
        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = bodies_storage.get(*$body);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();

        let $body =
            ServersStorage::<N>::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();
    };
}

macro_rules! extract_rigid_body_mut {
    ($_self:ident, $body:ident) => {
        let mut bodies_storage = $_self.storages.rbodies_w();
        let mut worlds_storage = $_self.storages.worlds_w();

        let $body = bodies_storage.get_mut(*$body);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();

        let $body =
            ServersStorage::<N>::rigid_body_mut($body.body_handle, *$body.world_tag, &mut worlds_storage);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();
    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {
        let bodies_storage = $_self.storages.rbodies_w();
        let worlds_storage = $_self.storages.worlds_w();

        let $body = bodies_storage.get(*$body);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();

        let $body =
            ServersStorage::<N>::rigid_body_mut($body.body_handle, *$body.world_tag, &mut worlds_storage);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();
    };
}

impl<N: RealField> RBodyNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        RBodyNpServer { storages }
    }
}

// This is a collection of function that can be used by other servers
impl<N: RealField> RBodyNpServer<N> {
    pub fn destroy_collider(body: &mut RigidBody, world: &mut NpWorld<N>) {
        fail_cond!(body.collider_handle.is_none());
        world.remove_colliders(&[body.collider_handle.unwrap()]);
        body.collider_handle = None;
    }

    pub fn copy_collider_desc(
        np_rigid_body: &mut NpRigidBody<N>,
        collider_desc: &mut NpColliderDesc<N>,
    ) {
        collider_desc.set_density(nalgebra::convert(1.0));
    }

    pub fn set_collider<'w>(
        body: &mut RigidBody,
        np_part_handle: NpBodyPartHandle,
        np_world: &'w mut NpWorld<N>,
        collider_desc: &NpColliderDesc<N>,
    ) {
        let collider = collider_desc.build_with_parent(np_part_handle, np_world);

        // Collider registration
        body.collider_handle = Some(collider.unwrap().handle());
    }
}

impl<N> RBodyPhysicsServerTrait<N> for RBodyNpServer<N>
where
    N: RealField,
    amethyst_core::Float: std::convert::From<N>,
    amethyst_core::Float: std::convert::Into<N>,
    N: alga::general::SubsetOf<amethyst_core::Float>,
{
    fn create_body(
        &mut self,
        world_tag: PhysicsWorldTag,
        body_desc: &RigidBodyDesc<N>,
    ) -> PhysicsBodyTag {
        let mut world_storage = self.storages.worlds_w();
        let mut bodies_storage = self.storages.rbodies_w();
        let mut shape_storage = self.storages.shapes_w();

        let np_world = world_storage.get_mut(*world_tag);
        assert!(np_world.is_some());
        let np_world = np_world.unwrap();

        let (rb_tag, rb_part_handle) = {
            // Create Rigid body
            let np_rigid_body = NpRigidBodyDesc::new()
                .set_position(TransfConversor::to_physics(&body_desc.transformation))
                .set_status(body_mode_conversor::to_physics(body_desc.mode))
                .set_mass(body_desc.mass)
                .build(np_world);

            let rb_part_handle = np_rigid_body.part_handle();
            (
                PhysicsBodyTag(
                    bodies_storage.make_opaque(RigidBody::new(np_rigid_body.handle(), world_tag)),
                ),
                rb_part_handle,
            )
        };
        let body = bodies_storage.get_mut(*rb_tag).unwrap();

        // Create and attach the collider
        let mut shape = shape_storage
            .get_mut(*body_desc.shape)
            .expect("During rigid body creation was not possible to find the shape");
        let mut collider_desc =
            NpColliderDesc::new(shape.shape_handle().clone()).density(nalgebra::convert(1.0));

        RBodyNpServer::set_collider(body, rb_part_handle, np_world, &collider_desc);

        // Collider registration
        shape.register_body(rb_tag);
        body.shape_tag = Some(body_desc.shape);

        rb_tag
    }

    fn drop_body(&mut self, body: PhysicsBodyTag) {
        unimplemented!();
        self.storages.rbodies_w().destroy(*body);
    }

    fn body_transform(&self, body: PhysicsBodyTag) -> Transform {
        extract_rigid_body!(self, body, Transform::default());

        TransfConversor::from_physics(body.position())
    }

    fn apply_force(&mut self, body: PhysicsBodyTag, force: &Vector3<N>){
        extract_rigid_body_mut!(self, body);

        body.apply_force(0, &Force::linear(*force), ForceType::Force, true);
    }
}
