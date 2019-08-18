//! The servers are the `Phythyst` interfaces, that is possible to use in order to control ary physics
//! engine that implements them.
//!
//! Each server controls a specific part of the physics engine, and they are:
//! - [World Server](trait.WorldPhysicsServerTrait.html)
//! - [RBody Server](trait.RBodyPhysicsServerTrait.html)
//! - [Area Server](trait.AreaPhysicsServerTrait.html)
//! - [Shape Server](trait.ShapePhysicsServerTrait.html)
//!
//! Is it possible to access them trough the `PhysicsWorld`.

mod area_server;
mod body_server;
mod shape_server;
mod world_server;

pub use area_server::*;
pub use body_server::{BodyMode, RBodyPhysicsServerTrait, RigidBodyDesc};
pub use shape_server::{ShapeDesc, ShapePhysicsServerTrait};
pub use world_server::WorldPhysicsServerTrait;

/// This struct contains all the servers that can be used to control a `PhysicsEngine`.
///
/// The `PhysicsWorld` is safe to be sent through threads because internally each `Backend` make sure
/// to access each data in thread safe.
pub struct PhysicsWorld<N>{
    world_server: Box<dyn WorldPhysicsServerTrait<N>>,
    rigid_body_server: Box<dyn RBodyPhysicsServerTrait<N>>,
    area_server: Box<dyn AreaPhysicsServerTrait>,
    shape_server: Box<dyn ShapePhysicsServerTrait<N>>,
}

impl<N> PhysicsWorld<N> {
    pub fn new(
        world_server: Box<dyn WorldPhysicsServerTrait<N>>,
        rigid_body_server: Box<dyn RBodyPhysicsServerTrait<N>>,
        area_server: Box<dyn AreaPhysicsServerTrait>,
        shape_server: Box<dyn ShapePhysicsServerTrait<N>>,
    ) -> Self {
        PhysicsWorld {
            world_server,
            rigid_body_server,
            area_server,
            shape_server,
        }
    }

    pub fn world_server(&self) -> &dyn WorldPhysicsServerTrait<N> {
        self.world_server.as_ref()
    }

    pub fn rigid_body_server(&self) -> &dyn RBodyPhysicsServerTrait<N> {
        self.rigid_body_server.as_ref()
    }

    pub fn area_server(&self) -> &dyn AreaPhysicsServerTrait {
        self.area_server.as_ref()
    }

    pub fn shape_server(&self) -> &dyn ShapePhysicsServerTrait<N> {
        self.shape_server.as_ref()
    }
}

unsafe impl<N> Send for PhysicsWorld<N> {}
unsafe impl<N> Sync for PhysicsWorld<N> {}
