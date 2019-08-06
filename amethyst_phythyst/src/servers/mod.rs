macro_rules! define_server {
    ($x:ident, $y:ident) => {
        /// This is a wrapper to the object that provide access to the $y functionalities.
        ///
        /// Check the function `amethyst_nphysics::create_physics` doc to see how.
        pub struct $x(pub Box<dyn $y>);

        unsafe impl Send for $x {}
        unsafe impl Sync for $x {}

        impl std::ops::Deref for $x {
            type Target = Box<dyn $y>;

            fn deref(&self) -> &Box<dyn $y> {
                &self.0
            }
        }

        impl std::ops::DerefMut for $x {
            fn deref_mut(&mut self) -> &mut Box<dyn $y> {
                &mut self.0
            }
        }
    };
}

macro_rules! define_server_generic {
    ($x:ident, $y:ident) => {
        /// This is a wrapper to the object that provide access to the $y functionalities.
        ///
        /// Check the function `amethyst_nphysics::create_physics` doc to see how.
        pub struct $x<N>(pub Box<dyn $y<N>>);

        unsafe impl<N> Send for $x<N> {}
        unsafe impl<N> Sync for $x<N> {}

        impl<N> std::ops::Deref for $x<N> {
            type Target = Box<dyn $y<N>>;

            fn deref(&self) -> &Box<dyn $y<N>> {
                &self.0
            }
        }

        impl<N> std::ops::DerefMut for $x<N> {
            fn deref_mut(&mut self) -> &mut Box<dyn $y<N>> {
                &mut self.0
            }
        }
    };
}

mod area_server;
mod body_server;
mod shape_server;
mod world_server;

pub use area_server::*;
pub use body_server::{BodyMode, RBodyPhysicsServerTrait, RigidBodyDesc};
pub use shape_server::{ShapeDesc, ShapePhysicsServerTrait};
pub use world_server::WorldPhysicsServerTrait;

define_server_generic!(WorldPhysicsServer, WorldPhysicsServerTrait);
define_server_generic!(RBodyPhysicsServer, RBodyPhysicsServerTrait);
define_server!(AreaPhysicsServer, AreaPhysicsServerTrait);
define_server_generic!(ShapePhysicsServer, ShapePhysicsServerTrait);

pub type PhysicsServers<N> = (
    WorldPhysicsServer<N>,
    RBodyPhysicsServer<N>,
    AreaPhysicsServer,
    ShapePhysicsServer<N>,
);
