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

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all)]

#[macro_use]
mod conditional_macros;
/*
#[macro_use]
mod utils;
mod area;
mod area_physics_server;
mod shape;
mod shape_physics_server;
*/
pub mod conversors;
pub mod storage;
#[macro_use]
mod servers_storage;
mod world;
mod world_physics_server;
mod body_storage;
mod rigid_body;
mod collider;
mod collider_storage;
mod joint;
mod joint_storage;
mod force_generator;
mod force_generator_storage;
//mod rigid_body_physics_server;
/*

use amethyst_phythyst::{
    servers::{
        AreaPhysicsServer, PhysicsServers, RBodyPhysicsServer, ShapePhysicsServer,
        WorldPhysicsServer,
    },
    PtReal,
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
*/
