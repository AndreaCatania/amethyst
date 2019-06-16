use amethyst_phythyst::servers::{
    WorldHandle, WorldServer
};

pub struct NWorldServer;

impl WorldServer for NWorldServer{
    fn create_world(&self) -> WorldHandle {
        println!("Hello from NPhysics server");
        WorldHandle(0)
    }
}