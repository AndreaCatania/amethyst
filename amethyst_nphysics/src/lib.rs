//! IMPORTANT:
//! This library is not meant to stay inside the amethyst project.
//! 
//! Actually this is here only to make it more simple to develop.
//! The idea is to move this outside once it's almost done.

//! This is the default Amethyst physics engine, to use this you have to
//! simply register as resource the object returned by `create_physics` of this
//! crate.
//! 
//! Follow the instructions of Phythyst to make more info about it.

mod world_server;

use amethyst_phythyst::Physics;
use world_server::NWorldServer;

/// This function returns an object that wrap all the functionalities required
/// by Phythyst.
/// 
/// Register this object as resource to allow Amethyst to use NPhysics.
pub fn create_physics() -> Physics {
    Physics {
        world_server: Box::new(NWorldServer::new()),
    }
}

