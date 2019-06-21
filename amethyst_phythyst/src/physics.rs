
use crate::servers::WorldServer;
use std::ops::{Deref, DerefMut};

pub type Physics = (PhysicsWorldServer, Option<i32>);

/// This wrapper contains all physics servers and is the access points that the
/// Systems will use.
/// 
/// Each Physics engine that you want to use in amethyst need to fill this object,
/// check the function `create_physics` in the amethyst_nphysics crate to see how.
pub struct PhysicsWorldServer(pub Box<dyn WorldServer>);

unsafe impl Send for PhysicsWorldServer {}
unsafe impl Sync for PhysicsWorldServer {}

impl Deref for PhysicsWorldServer{

    type Target = Box<dyn WorldServer>;

    fn deref(&self) -> &Box<dyn WorldServer>{
        &self.0
    }
}

impl DerefMut for PhysicsWorldServer{

    fn deref_mut(&mut self) -> &mut Box<dyn WorldServer>{
        &mut self.0
    }
}

