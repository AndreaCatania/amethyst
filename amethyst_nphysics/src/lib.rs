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
pub mod conversors;
pub mod storage;
mod utils;
#[macro_use]
mod servers_storage;
mod area_physics_server;
mod body;
mod body_storage;
mod collider_storage;
mod force_generator;
mod force_generator_storage;
mod joint;
mod joint_storage;
mod rigid_body_physics_server;
mod shape;
mod shape_physics_server;
mod world_physics_server;

use area_physics_server::AreaNpServer;
use rigid_body_physics_server::RBodyNpServer;
use shape_physics_server::ShapeNpServer;
use world_physics_server::WorldNpServer;

use amethyst_phythyst::{
    servers::{
        AreaPhysicsServerTrait, PhysicsWorld, RBodyPhysicsServerTrait, ShapePhysicsServerTrait,
    },
    PtReal,
};

pub struct NPhysicsBackend;

/// NPhysics Backend
impl<N> amethyst_phythyst::PhysicsBackend<N> for NPhysicsBackend
where
    N: PtReal,
{
    fn create_world() -> PhysicsWorld<N> {
        let storages = servers_storage::ServersStorage::new();

        PhysicsWorld::new(
            Box::new(WorldNpServer::new(storages.clone())),
            Box::new(RBodyNpServer::new(storages.clone())),
            Box::new(AreaNpServer::new(storages.clone())),
            Box::new(ShapeNpServer::new(storages.clone())),
        )
    }
}
