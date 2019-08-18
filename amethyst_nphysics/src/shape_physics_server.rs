use amethyst_phythyst::{
    objects::*,
    servers::{ShapeDesc, ShapePhysicsServerTrait},
    PtReal,
};
use log::error;
use nphysics3d::object::ColliderDesc as NpColliderDesc;

use crate::{
    area_physics_server::AreaNpServer, conversors::*, rigid_body::BodyData,
    rigid_body_physics_server::RBodyNpServer, servers_storage::*, shape::RigidShape,
    storage::StoreKey,
};

pub struct ShapeNpServer<N: PtReal> {
    storages: ServersStorageType<N>,
}

impl<N: PtReal> ShapeNpServer<N> {
    pub fn new(storages: ServersStorageType<N>) -> Self {
        ShapeNpServer { storages }
    }

    /// Drop a shape, return false if it can't be removed right now or it something failed.
    pub fn drop_shape(
        shape_tag: PhysicsShapeTag,
        shapes_storage: &mut ShapeStorageWrite<N>,
    ) -> bool {
        let shape_key = tag_to_store_key(shape_tag.0);

        let safe_to_drop = !ShapeNpServer::has_dependency(shape_key, shapes_storage);

        if !safe_to_drop {
            if let Some(shape) = shapes_storage.get_mut(shape_key) {
                if !shape.marked_for_drop {
                    shape.marked_for_drop = true;
                    fail!("A shape is marked for drop while still in use. Consider to store the PhysicsHandle<PhysicsShapeTag> to not waste resources.", false);
                }
            }
            false
        } else {
            shapes_storage.destroy(shape_key);
            true
        }
    }

    /// Returns `true` if this shape is still in use.
    pub fn has_dependency(shape_key: StoreKey, shapes_storage: &mut ShapeStorageWrite<N>) -> bool {
        if let Some(shape) = shapes_storage.get_mut(shape_key) {
            if shape.bodies().len() > 0 {
                return true;
            }
        }

        false
    }
}

impl<N: PtReal> ShapePhysicsServerTrait<N> for ShapeNpServer<N> {
    fn create_shape(&mut self, shape_desc: &ShapeDesc<N>) -> PhysicsHandle<PhysicsShapeTag> {
        let shape = Box::new(RigidShape::new(shape_desc));

        let mut shapes_storage = self.storages.shapes_w();
        let mut shape_key = (shapes_storage.make_opaque(shape));

        let shape = shapes_storage.get_mut(shape_key).unwrap();
        shape.self_key = Some(shape_key);

        PhysicsHandle::new(PhysicsShapeTag(store_key_to_tag(shape_key)), self.storages.gc.clone())
    }

    fn update_shape(&mut self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>) {
        let mut bodies = self.storages.rbodies_w();
        let mut colliders = self.storages.colliders_w();
        let mut shapes = self.storages.shapes_w();

        let shape_key = tag_to_store_key(shape_tag.0);
        if let Some(shape) = shapes.get_mut(shape_key) {
            shape.update(shape_desc);

            let b_keys = shape.bodies();
            for body_key in b_keys {
                if let Some(body) = bodies.get_body_mut(*body_key) {
                    let mut collider_desc = NpColliderDesc::new(shape.shape_handle().clone());

                    match &body.body_data {
                        BodyData::Rigid => {
                            RBodyNpServer::drop_collider(body, &mut colliders);
                            RBodyNpServer::extract_collider_desc(body.rigid_body().unwrap(), &mut collider_desc);
                            RBodyNpServer::install_collider(body, &collider_desc, &mut colliders);
                        }
                        BodyData::Area(e) => {
                            AreaNpServer::drop_collider(body, &mut colliders);
                            AreaNpServer::extract_collider_desc(body.rigid_body().unwrap(), &mut collider_desc);
                            AreaNpServer::install_collider(body, &collider_desc, &mut colliders);
                        }
                    }
                }
            }
        } else {
            error!("Shape not found!");
        }
    }
}
