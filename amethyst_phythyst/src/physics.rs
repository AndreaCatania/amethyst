
use crate::servers::{
    WorldServer,
    BodyServer,
};
use std::ops::{Deref, DerefMut};

pub type Physics = (PhysicsWorldServer, PhysicsBodyServer);


#[macro_export]
macro_rules! define_server{
    ($x:ident, $y:ident) => {
        /// This is a wrapper to the object that provide access to the $y functionalities.
        ///
        /// Check the function `amethyst_nphysics::create_physics` doc to see how.
        pub struct $x(pub Box<dyn $y>);

        unsafe impl Send for $x {}
        unsafe impl Sync for $x {}

        impl Deref for $x{

            type Target = Box<dyn $y>;

            fn deref(&self) -> &Box<dyn $y>{
                &self.0
            }
        }

        impl DerefMut for $x{

            fn deref_mut(&mut self) -> &mut Box<dyn $y>{
                &mut self.0
            }
        }
    }
}

define_server!(PhysicsWorldServer, WorldServer);
define_server!(PhysicsBodyServer, BodyServer);
