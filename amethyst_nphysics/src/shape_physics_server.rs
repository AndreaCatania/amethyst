use nphysics3d::object::ColliderDesc as NpColliderDesc;
use amethyst_phythyst::{
    objects::*,
    servers::{ShapeDesc, ShapePhysicsServerTrait},
    PtReal,
};

use crate::{
    area_physics_server::AreaNpServer, conversors::*, rigid_body_physics_server::RBodyNpServer,
    servers_storage::*, shape::RigidShape, storage::StoreKey,
};


pub struct ShapeNpServer<N: PtReal> {
    storages: ServersStorageType<N>,
}

impl<N: PtReal> ShapeNpServer<N> {
    //pub fn new(storages: ServersStorageType<N>) -> Self {
    //    ShapeNpServer { storages }
    //}

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
        }else{

            shapes_storage.destroy(shape_key);
            true
        }
    }

    /// Returns `true` if this shape is still in use.
    pub fn has_dependency(
        shape_key: StoreKey,
        shapes_storage: &mut ShapeStorageWrite<N>,
    ) -> bool {

        if let Some(shape) = shapes_storage.get_mut(shape_key) {

            if shape.bodies().len() > 0 {
                return true;
            }

            if shape.areas().len() > 0 {
                return true;
            }
        }

        false
    }
}
/*
impl<N: PtReal> ShapePhysicsServerTrait<N> for ShapeNpServer<N> {
    fn create_shape(&mut self, shape_desc: &ShapeDesc<N>) -> PhysicsHandle<PhysicsShapeTag> {
        let shape = Box::new(RigidShape::new(shape_desc));

        let mut shapes_storage = self.storages.shapes_w();
        let mut shape_tag = PhysicsShapeTag(shapes_storage.make_opaque(shape));

        let shape = shapes_storage.get_mut(*shape_tag).unwrap();
        shape.self_tag = Some(shape_tag);

        PhysicsHandle::new(shape_tag, self.storages.gc.clone())
    }

    fn update_shape(&mut self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>) {
        let mut worlds_storage = self.storages.worlds_w();
        let mut bodies_storage = self.storages.rbodies_w();
        let mut areas_storage = self.storages.areas_w();
        let mut shapes_storage = self.storages.shapes_w();

        let shape = storage_safe_get_mut!(shapes_storage, shape_tag);

        shape.update(shape_desc);

        // Phase 1. Update the shapes of all bodies.
        {
            let bodies = shape.bodies();
            for body_tag in bodies {
                let body = bodies_storage.get_mut(**body_tag);
                if let Some(body) = body {
                    let np_world = worlds_storage.get_mut(*body.world_tag);
                    if let Some(np_world) = np_world {
                        RBodyNpServer::drop_collider(body, np_world);

                        if let Some(np_body) = np_world.rigid_body_mut(body.body_handle) {
                            let mut collider_desc =
                                NpColliderDesc::new(shape.shape_handle().clone());
                            RBodyNpServer::extract_collider_desc(np_body, &mut collider_desc);

                            RBodyNpServer::install_collider(
                                body,
                                np_body.part_handle(),
                                np_world,
                                &collider_desc,
                            );
                        }
                    }
                }
            }
        }

        // Phase 2. Update the shapes of all areas.
        {
            let areas = shape.areas();
            for area_tag in areas {
                let area = areas_storage.get_mut(**area_tag);
                if let Some(area) = area {
                    let np_world = worlds_storage.get_mut(*area.world_tag);
                    if let Some(np_world) = np_world {
                        let mut collider_desc = NpColliderDesc::new(shape.shape_handle().clone());

                        if let Some(np_collider) =
                            np_world.collider_mut(area.collider_handle.unwrap())
                        {
                            AreaNpServer::copy_collider_desc(np_collider, &mut collider_desc);
                        } else {
                            panic!("Just found an area without collider. This must never happen. Please report this.");
                        }

                        AreaNpServer::destroy_collider(area, np_world);

                        AreaNpServer::set_collider(area, *area_tag, np_world, &collider_desc);
                    }
                }
            }
        }
    }
}
*/