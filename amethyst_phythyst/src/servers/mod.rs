
#[macro_export]
macro_rules! define_server{
    ($x:ident, $y:ident) => {
        /// This is a wrapper to the object that provide access to the $y functionalities.
        ///
        /// Check the function `amethyst_nphysics::create_physics` doc to see how.
        pub struct $x(pub Box<dyn $y>);

        unsafe impl Send for $x {}
        unsafe impl Sync for $x {}

        impl std::ops::Deref for $x{

            type Target = Box<dyn $y>;

            fn deref(&self) -> &Box<dyn $y>{
                &self.0
            }
        }

        impl std::ops::DerefMut for $x{

            fn deref_mut(&mut self) -> &mut Box<dyn $y>{
                &mut self.0
            }
        }
    }
}

mod world_server;
mod body_server;

pub use world_server::WorldPhysicsServerTrait;
pub use body_server::{
    RBodyPhysicsServerTrait,
    RigidBodyDesc,
};

define_server!(WorldPhysicsServer, WorldPhysicsServerTrait);
define_server!(RBodyPhysicsServer, RBodyPhysicsServerTrait);

pub type PhysicsServers = (WorldPhysicsServer, RBodyPhysicsServer);