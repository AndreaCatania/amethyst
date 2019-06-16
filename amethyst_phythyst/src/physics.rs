

use crate::servers::WorldServer;

/// This wrapper contains all physics servers and is the access points that the
/// Systems will use.
/// 
/// Each Physics engine that you want to use in amethyst need to fill this object,
/// check the function `create_physics` in the amethyst_nphysics crate to see how.
pub struct Physics{
    pub world_server: Box<dyn WorldServer>,
}

unsafe impl Send for Physics {}
unsafe impl Sync for Physics {}

