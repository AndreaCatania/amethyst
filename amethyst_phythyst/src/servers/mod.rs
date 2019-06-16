mod memory;
mod world_server;

pub use memory::Memory;

pub use world_server::{
    WorldHandle,
    WorldServer,
};


/// The physics handle to an object of the engine
pub struct PhyHandle(pub std::num::NonZeroUsize);