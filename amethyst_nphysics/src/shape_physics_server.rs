use amethyst_phythyst::{
    objects::*,
    servers::{ShapeDesc, ShapePhysicsServerTrait},
};

use crate::{
    conversors::*, rigid_body_physics_server::RBodyNpServer, servers_storage::*, shape::RigidShape,
};

use nphysics3d::object::ColliderDesc as NpColliderDesc;

use nalgebra::RealField;

pub struct ShapeNpServer<N: RealField> {
    storages: ServersStorageType<N>,
}

impl<N: RealField> ShapeNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        ShapeNpServer { storages }
    }
}

impl<N: RealField> ShapePhysicsServerTrait<N> for ShapeNpServer<N> {
    fn create_shape(&mut self, shape_desc: &ShapeDesc<N>) -> PhysicsShapeTag {
        let shape = Box::new(RigidShape::new(shape_desc));

        PhysicsShapeTag(self.storages.shapes_w().make_opaque(shape))
    }

    fn drop_shape(&mut self, shape_tag: PhysicsShapeTag) {
        unimplemented!();
    }

    fn update_shape(&mut self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>) {
        let mut worlds_storage = self.storages.worlds_w();
        let mut bodies_storage = self.storages.rbodies_w();
        let mut shapes_storage = self.storages.shapes_w();

        let shape = shapes_storage.get_mut(*shape_tag);
        fail_cond!(shape.is_none());
        let mut shape = shape.unwrap();

        shape.update(shape_desc);

        // Phase 1. Update the shapes of all bodies.
        let bodies = shape.bodies();
        for body_tag in bodies {
            let body = bodies_storage.get_mut(**body_tag);
            if let Some(body) = body {
                let np_world = worlds_storage.get_mut(*body.world_tag);
                if let Some(np_world) = np_world {
                    RBodyNpServer::destroy_collider(body, np_world);

                    if let Some(np_body) = np_world.rigid_body_mut(body.body_handle) {
                        let mut collider_desc = NpColliderDesc::new(shape.shape_handle().clone());
                        RBodyNpServer::copy_collider_desc(np_body, &mut collider_desc);

                        RBodyNpServer::set_collider(
                            body,
                            np_body.part_handle(),
                            np_world,
                            &collider_desc,
                        );
                    }
                }
            }
        }

        // Phase 2. Update the shapes of all areas.
        // TODO please make sure to reload the area shape
        unimplemented!();
    }
}
