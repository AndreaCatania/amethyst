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

use nalgebra::{Isometry3, Point, Vector, Vector3};

use amethyst_phythyst::{PtReal, objects::*, servers::*};

use amethyst_core::ecs::Entity;

use crate::{
    conversors::*, rigid_body::RigidBody, servers_storage::*, shape::RigidShape, utils::*,
};

pub struct RBodyNpServer<N: PtReal> {
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

impl<N: PtReal> RBodyNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        RBodyNpServer { storages }
    }
}

// This is a collection of function that can be used by other servers to perform some common
// operations on areas.
impl<N: PtReal> RBodyNpServer<N> {
    pub fn drop_body(
        body_tag: PhysicsBodyTag,
        worlds_storage: &mut WorldStorageWrite<N>,
        rbodies_storage: &mut RigidBodyStorageWrite,
        shapes_storage: &mut ShapeStorageWrite<N>,
    ) {
        {
            let body = storage_safe_get_mut!(rbodies_storage, body_tag);

            // Remove from shape
            let shape = storage_safe_get_mut!(shapes_storage, body.shape_tag.unwrap());

            // Remove from world
            let world = storage_safe_get_mut!(worlds_storage, body.world_tag);
            RBodyNpServer::remove_shape(shape, body, world);
            world.remove_bodies(&[body.body_handle]);
        }

        rbodies_storage.destroy(*body_tag);
    }

    /// Set shape.
    /// Take care to register the shape and set the collider to the body.
    pub fn install_shape<'w>(
        body: &mut RigidBody,
        np_part_handle: NpBodyPartHandle,
        np_world: &'w mut NpWorld<N>,
        shape: &mut RigidShape<N>,
        collider_desc: &NpColliderDesc<N>,
    ) {
        Self::install_collider(body, np_part_handle, np_world, collider_desc);

        // Collider registration
        shape.register_body(body.self_tag.unwrap());
        body.shape_tag = shape.self_tag;
    }

    /// Remove shape.
    /// Take care to unregister the shape and then drop the internal collider.
    pub fn remove_shape(shape: &mut RigidShape<N>, body: &mut RigidBody, world: &mut NpWorld<N>) {
        Self::drop_collider(body, world);
        shape.unregister_body(body.self_tag.unwrap());
    }

    /// Set collider to the body
    pub fn install_collider<'w>(
        body: &mut RigidBody,
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

    /// Just drop the internal collider of the passed body.
    pub fn drop_collider(body: &mut RigidBody, world: &mut NpWorld<N>) {
        if body.collider_handle.is_none() {
            return;
        }
        world.remove_colliders(&[body.collider_handle.unwrap()]);
        body.collider_handle = None;
    }

    pub fn update_user_data(collider: &mut NpCollider<N>, body: &RigidBody) {
        collider.set_user_data(Some(Box::new(UserData::new(
            ObjectType::RigidBody,
            body.self_tag.unwrap().0,
            body.entity,
        ))));
    }

    /// Extract collider description from a rigid body
    pub fn extract_collider_desc(
        np_rigid_body: &mut NpRigidBody<N>,
        collider_desc: &mut NpColliderDesc<N>,
    ) {
        collider_desc.set_density(nalgebra::convert(1.0));
    }
}

/// ### Serial execution
/// There are functions that are marked as serial execution.
/// These functions doesn't have the capacity to be executed in parallel. Even if executed by different
/// threads.
impl<N> RBodyPhysicsServerTrait<N> for RBodyNpServer<N>
where
    N: PtReal,
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

    fn set_shape(&self, body_tag: PhysicsBodyTag, shape_tag: Option<PhysicsShapeTag>) {
        let mut body_storage = self.storages.rbodies_w();
        let body = storage_safe_get_mut!(body_storage, body_tag);

        if body.shape_tag == shape_tag {
            return;
        }

        let mut world_storage = self.storages.worlds_w();
        let np_world = storage_safe_get_mut!(world_storage, body.world_tag);

        let mut shape_storage = self.storages.shapes_w();

        if body.shape_tag.is_some() {
            let shape = shape_storage.get_mut(*body.shape_tag.unwrap()).unwrap();
            RBodyNpServer::remove_shape(shape, body, np_world);
        }

        if let Some(shape_tag) = shape_tag {
            // Create and attach the collider
            let mut shape = shape_storage
                .get_mut(*shape_tag)
                .expect("During rigid body creation was not possible to find the shape");

            let mut collider_desc =
                NpColliderDesc::new(shape.shape_handle().clone()).density(nalgebra::convert(1.0));

            let body_part_handle = {
                let np_rigid_body = np_world.rigid_body_mut(body.body_handle).unwrap();
                np_rigid_body.part_handle()
            };

            RBodyNpServer::install_shape(body, body_part_handle, np_world, shape, &collider_desc);
        }
    }

    fn shape(&self, body_tag: PhysicsBodyTag) -> Option<PhysicsShapeTag> {
        let mut body_storage = self.storages.rbodies_r();
        let body = storage_safe_get!(body_storage, body_tag, None);
        body.shape_tag
    }

    fn set_body_transform(&self, body_tag: PhysicsBodyTag, transf: &Isometry3<f32>) {
        let mut bodies_storage = self.storages.rbodies_w();
        let mut worlds_storage = self.storages.worlds_w();

        let body = storage_safe_get!(bodies_storage, body_tag);
        let world = storage_safe_get_mut!(worlds_storage, body.world_tag);

        let transf = TransfConversor::to_physics(transf);

        if body.collider_handle.is_some() {
            // TODO, There is a bug that affect the version 0.11.1, that doesn't update the transform
            // on the colliders when the position of the RigidBody is changed.
            // So I'm doing it manually.
            // But please remove it, once this bug is fixed.
            {
                // TODO remove this if the actual NPhysics got updated since actually there' a bug (v0.11.1)
                world
                    .collider_world_mut()
                    .set_position(body.collider_handle.unwrap(), transf.clone());
            }

            // TODO this si not required when the bug
            if body.body_mode != BodyMode::Dynamic {
                // Set the position of the collider, this is necessary for static objects
                let np_collider = world.collider_mut(body.collider_handle.unwrap());
                fail_cond!(np_collider.is_none());
                let np_collider = np_collider.unwrap();
                np_collider.set_position(transf.clone());
            }
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
