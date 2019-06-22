
//! # Phythyst
//! Phythyst is responsible to control a Physics Engine and make it easily
//! usable in Amethyst.
//! 
//! It defines the actions that a physics engine can do, like `create_world`
//! or `set_gravity` and so on.
//! But doesn't implement any of these, infact it leaves this task to the
//! physics engine.
//! 
//! In this way is possible to integrate any kind of physics engine in Amethyst,
//! by simply implementing the actions that this crate define.
//! This mean that you can easily swap Physics Engine at any point of your game
//! development allowing you to choose the more stable depending on your Game!
//! 
//! But let's start.
//! 
//! # How to use Phythyst?
//! Use Phythyst is really simple. You just need to initialize it.
//! 
//! The first thing to do is register the PhysicsBundle:
//! ```
//! use amethyst::phythyst::PhysicsBundle;
//! 
//! let game_data = GameDataBuilder::default()
//!     .with_bundle(PhysicsBundle::default()).unwrap()
//! 
//! ```
//! 
//! Now you need to splicify the Physics Engine that you want to use.
//! The default physics engine in Amethyst is NPhysics, and to use it you have
//! to register as resource the object returned by the function `create_physics`
//! under the amethyst_nphysics crate.
//! ```
//! let mut game = Application::build("./", GameState)?
//!     .with_resource(amethyst_nphysics::create_physics())
//! ```
//! 
//! That's it!
//! From now on you are able to use physics in Amethyst.
//! And remember, to change the physics engine you **just** need to simply register
//! another Physics resource.
//! 
//! **Enjoy!**
//! 

mod systems;
mod physics_bundle;
mod physics;
mod physics_time;

pub mod servers;

pub use physics_bundle::PhysicsBundle;
pub use physics::{Physics, PhysicsWorldServer, PhysicsRigidBodyServer};
pub use physics_time::PhysicsTime;
use crate::servers::PhysicsBodyTag;
