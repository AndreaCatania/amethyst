//! # IMPORTANT:
//! This library is not meant to stay inside the amethyst project.
//!
//! Actually this is here only to make it more simple to develop.
//! The idea is to move this outside once it's almost done.

//! # Amethyst default physics engine
//! This is the default Amethyst physics engine.
//! To use this you have to register the `PhysicsServers` returned by the fn `create_physics`
//! that is located in this crate, using the fn `Application::with_physics`.
//!
//! Follow the instructions of Phythyst to make more info about it.
//!
//! # Dev info
//!
//! ## Naming
//! Since NPhysics doesn't use any prefix to identify its structures this implementation take care
//! to append the prefix `Np` to any struct that come from NPhysics.
//! In this way is possible to have a clear distinction of what is what.
//! Example: `RigidBody` and `NpRigidBody`.

#[macro_use]
mod conditional_macros;
#[macro_use]
mod servers_storage;
#[macro_use]
mod utils;

mod area;
mod area_physics_server;
mod conversors;
mod rigid_body;
mod rigid_body_physics_server;
mod shape;
mod shape_physics_server;
mod storage;
mod world;
mod world_physics_server;

use amethyst_phythyst::{
    PtReal,
    servers::{
        AreaPhysicsServer, PhysicsServers, RBodyPhysicsServer, ShapePhysicsServer, WorldPhysicsServer,
    }
};
use area_physics_server::*;
use nalgebra::RealField;
use rigid_body_physics_server::RBodyNpServer;
use shape_physics_server::*;
use world_physics_server::WorldNpServer;

pub struct NPhysicsBackend;

/// NPhysics Backend
impl<N> amethyst_phythyst::PhysicsBackend<N> for NPhysicsBackend
where
    N: PtReal,
{
    fn create_servers() -> PhysicsServers<N> {
        let storages = servers_storage::ServersStorage::new();

        (
            WorldPhysicsServer(Box::new(WorldNpServer::new(storages.clone()))),
            RBodyPhysicsServer(Box::new(RBodyNpServer::new(storages.clone()))),
            AreaPhysicsServer(Box::new(AreaNpServer::new(storages.clone()))),
            ShapePhysicsServer(Box::new(ShapeNpServer::new(storages.clone()))),
        )
    }
}
