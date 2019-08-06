use amethyst_phythyst::{objects::*, servers::*};

use amethyst_core::ecs::Entity;

use nphysics3d::{
    algebra::Velocity3,
    math::{Force, ForceType, Velocity},
    object::{
        Body as NpBody, BodyHandle as NpBodyHandle, BodyPartHandle as NpBodyPartHandle,
        BodyStatus as NpBodyStatus, Collider as NpCollider, ColliderDesc as NpColliderDesc,
        RigidBody as NpRigidBody, RigidBodyDesc as NpRigidBodyDesc,
    },
    world::World as NpWorld,
};

use ncollide3d::shape::{Ball as NcBall, ShapeHandle as NcShapeHandle};

use nalgebra::{Isometry3, Point, RealField, Vector, Vector3};

use crate::{conversors::*, rigid_body::RigidBody, servers_storage::*, utils::*};

pub struct RBodyNpServer<N: RealField> {
    storages: ServersStorageType<N>,
}

macro_rules! extract_np_rigid_body {
    ($_self:ident, $body:ident) => {
        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = storage_safe_get!(bodies_storage, $body);

        let $body =
            ServersStorage::<N>::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none());
        let $body = $body.unwrap();
    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {
        let bodies_storage = $_self.storages.rbodies_r();
        let worlds_storage = $_self.storages.worlds_r();

        let $body = storage_safe_get!(bodies_storage, $body, $on_fail_ret);

        let $body =
            ServersStorage::<N>::rigid_body($body.body_handle, *$body.world_tag, &worlds_storage);
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();
    };
}

macro_rules! extract_np_rigid_body_mut {
    ($_self:ident, $body:ident) => {
        let mut bodies_storage = $_self.storages.rbodies_w();
        let mut worlds_storage = $_self.storages.worlds_w();

        let $body = storage_safe_get!(bodies_storage, $body);

        let $body = ServersStorage::<N>::rigid_body_mut(
            $body.body_handle,
            *$body.world_tag,
            &mut worlds_storage,
        );
        fail_cond!($body.is_none());
        let $body = $body.unwrap();
    };
    ($_self:ident, $body:ident, $on_fail_ret:expr) => {
        let bodies_storage = $_self.storages.rbodies_w();
        let worlds_storage = $_self.storages.worlds_w();

        let $body = storage_safe_get!(bodies_storage, $body, $on_fail_ret);

        let $body = ServersStorage::<N>::rigid_body_mut(
            $body.body_handle,
            *$body.world_tag,
            &mut worlds_storage,
        );
        fail_cond!($body.is_none(), $on_fail_ret);
        let $body = $body.unwrap();
    };
}

impl<N: RealField> RBodyNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        RBodyNpServer { storages }
    }
}

// This is a collection of function that can be used by other servers to perform some common
// operations on areas.
impl<N: RealField> RBodyNpServer<N> {
    pub fn drop_body(
        body_tag: PhysicsBodyTag,
        worlds_storage: &mut WorldStorageWrite<N>,
        rbodies_storage: &mut RigidBodyStorageWrite,
        shapes_storage: &mut ShapeStorageWrite<N>,
    ) {
        {
            let body = storage_safe_get!(rbodies_storage, body_tag);

            // Remove from world
            let world = storage_safe_get_mut!(worlds_storage, body.world_tag);
            if let Some(handle) = body.collider_handle {
                world.remove_colliders(&[handle]);
            }
            world.remove_bodies(&[body.body_handle]);

            // Remove from shape
            let shape = storage_safe_get_mut!(shapes_storage, body.shape_tag.unwrap());
            shape.unregister_body(body_tag);
        }

        rbodies_storage.destroy(*body_tag);
    }

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
        body_tag: PhysicsBodyTag,
        np_part_handle: NpBodyPartHandle,
        np_world: &'w mut NpWorld<N>,
        collider_desc: &NpColliderDesc<N>,
    ) {
        let collider = collider_desc
            .build_with_parent(np_part_handle, np_world)
            .unwrap();

        RBodyNpServer::update_user_data(collider, body);

        // Collider registration
        body.collider_handle = Some(collider.handle());
    }

    pub fn update_user_data(collider: &mut NpCollider<N>, body: &RigidBody) {
        collider.set_user_data(Some(Box::new(UserData::new(
            ObjectType::RigidBody,
            body.self_tag.unwrap().0,
            body.entity,
        ))));
    }
}

/// ### Serial execution
/// There are functions that are marked as serial execution.
/// These functions doesn't have the capacity to be executed in parallel. Even if executed by different
/// threads.
impl<N> RBodyPhysicsServerTrait<N> for RBodyNpServer<N>
where
    N: RealField,
    N: std::convert::From<f32>,
    f32: From<N>,
{
    fn create_body(
        &mut self,
        world_tag: PhysicsWorldTag,
        body_desc: &RigidBodyDesc<N>,
    ) -> PhysicsHandle<PhysicsBodyTag> {
        let mut world_storage = self.storages.worlds_w();
        let mut bodies_storage = self.storages.rbodies_w();
        let mut shape_storage = self.storages.shapes_w();

        let np_world = world_storage
            .get_mut(*world_tag)
            .expect("During the rigid body creation the world tag passed was not valid");

        let rb_tag =
            PhysicsBodyTag(bodies_storage.make_opaque(RigidBody::new(world_tag, body_desc.mode)));

        // Create Rigid body
        let np_rigid_body = NpRigidBodyDesc::new()
            .set_status(body_mode_conversor::to_physics(body_desc.mode))
            .set_mass(body_desc.mass)
            .build(np_world);

        let body = bodies_storage.get_mut(*rb_tag).unwrap();
        body.self_tag = Some(rb_tag);
        body.body_handle = np_rigid_body.handle();

        // Create and attach the collider
        let mut shape = shape_storage
            .get_mut(*body_desc.shape)
            .expect("During rigid body creation was not possible to find the shape");
        let mut collider_desc =
            NpColliderDesc::new(shape.shape_handle().clone()).density(nalgebra::convert(1.0));

        RBodyNpServer::set_collider(
            body,
            rb_tag,
            np_rigid_body.part_handle(),
            np_world,
            &collider_desc,
        );

        // Collider registration
        shape.register_body(rb_tag);
        body.shape_tag = Some(body_desc.shape);

        PhysicsHandle::new(rb_tag, self.storages.gc.clone())
    }

    fn set_entity(&self, body_tag: PhysicsBodyTag, entity: Option<Entity>) {
        let mut body_storage = self.storages.rbodies_w();
        let body = storage_safe_get_mut!(body_storage, body_tag);
        body.entity = entity;

        if body.collider_handle.is_none() {
            return;
        }
        let mut world_storage = self.storages.worlds_w();
        let world = storage_safe_get_mut!(world_storage, body.world_tag);
        let collider = world.collider_mut(body.collider_handle.unwrap()).unwrap();

        RBodyNpServer::update_user_data(collider, body);
    }

    fn entity(&self, body_tag: PhysicsBodyTag) -> Option<Entity> {
        let body_storage = self.storages.rbodies_r();
        let body = storage_safe_get!(body_storage, body_tag, None);
        body.entity
    }

    fn set_body_transform(&self, body_tag: PhysicsBodyTag, transf: &Isometry3<f32>) {
        let mut bodies_storage = self.storages.rbodies_w();
        let mut worlds_storage = self.storages.worlds_w();

        let body = storage_safe_get!(bodies_storage, body_tag);
        let world = storage_safe_get_mut!(worlds_storage, body.world_tag);

        let transf = TransfConversor::to_physics(transf);

        {
            // TODO remove this if the actual NPhysics got updated since actually there' a bug (v0.11.1)
            world
                .collider_world_mut()
                .set_position(body.collider_handle.unwrap(), transf.clone());
        }

        // TODO this should be not required check when the above bug is fixed
        if body.body_mode != BodyMode::Dynamic {
            // Set the position of the collider, this is necessary for static objects
            let np_collider = world.collider_mut(body.collider_handle.unwrap());
            fail_cond!(np_collider.is_none());
            let np_collider = np_collider.unwrap();
            np_collider.set_position(transf.clone());
        }

        {
            // Set the position of the rigid body
            let np_body = world.rigid_body_mut(body.body_handle);
            fail_cond!(np_body.is_none());
            let np_body = np_body.unwrap();

            np_body.set_position(transf);
        }
    }

    fn body_transform(&self, body: PhysicsBodyTag) -> Isometry3<f32> {
        extract_np_rigid_body!(self, body, Isometry3::identity());

        TransfConversor::from_physics(body.position())
    }

    fn clear_forces(&self, body: PhysicsBodyTag) {
        extract_np_rigid_body_mut!(self, body);

        body.clear_forces();
    }

    fn apply_force(&self, body: PhysicsBodyTag, force: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force(0, &Force::linear(*force), ForceType::Force, true);
    }

    fn apply_torque(&self, body: PhysicsBodyTag, force: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force(0, &Force::torque(*force), ForceType::Force, true);
    }

    fn apply_force_at_position(
        &self,
        body: PhysicsBodyTag,
        force: &Vector3<N>,
        position: &Vector3<N>,
    ) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force_at_point(0, force, &Point::from(*position), ForceType::Force, true);
    }

    fn apply_impulse(&self, body: PhysicsBodyTag, impulse: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force(0, &Force::linear(*impulse), ForceType::Impulse, true);
    }

    fn apply_angular_impulse(&self, body: PhysicsBodyTag, impulse: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force(0, &Force::torque(*impulse), ForceType::Impulse, true);
    }

    fn apply_impulse_at_position(
        &self,
        body: PhysicsBodyTag,
        impulse: &Vector3<N>,
        position: &Vector3<N>,
    ) {
        extract_np_rigid_body_mut!(self, body);

        body.apply_force_at_point(
            0,
            impulse,
            &Point::from(*position),
            ForceType::Impulse,
            true,
        );
    }

    fn set_linear_velocity(&self, body: PhysicsBodyTag, velocity: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.set_velocity(Velocity3::new(*velocity, body.velocity().angular));
    }

    fn linear_velocity(&self, body: PhysicsBodyTag) -> Vector3<N> {
        extract_np_rigid_body!(self, body, Vector3::zeros());

        body.velocity().linear
    }

    fn set_angular_velocity(&self, body: PhysicsBodyTag, velocity: &Vector3<N>) {
        extract_np_rigid_body_mut!(self, body);

        body.set_velocity(Velocity3::new(body.velocity().linear, *velocity));
    }

    fn angular_velocity(&self, body: PhysicsBodyTag) -> Vector3<N> {
        extract_np_rigid_body!(self, body, Vector3::zeros());

        body.velocity().angular
    }
}
