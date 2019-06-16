

use crate::servers::{PhysicsServer, WorldHandle};

/// This wrapper contains all physics servers and is the access points that the
/// Systems will use.
pub struct Physics{
    pub server: Box<dyn PhysicsServer>,
}

impl Default for Physics{
    fn default() -> Self {
        
        Physics{
            server: Box::new(DummyPhysicsServer{}),
        }
    }
}

unsafe impl Send for Physics {}
unsafe impl Sync for Physics {}

pub struct DummyPhysicsServer;

impl PhysicsServer for DummyPhysicsServer{
    fn create_world(&self) -> WorldHandle{
        WorldHandle(0)
    }
}