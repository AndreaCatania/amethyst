

use amethyst_phythyst::{
    servers::{
        PhysicsWorldTag,
        PhysicsBodyTag,
        BodyServer,
    },
};

pub struct NBodyServer{
}

impl NBodyServer{

    pub fn new() -> Self{
        NBodyServer{}
    }
}

impl BodyServer for NBodyServer{

    fn create(&mut self, world: PhysicsWorldTag) -> PhysicsBodyTag {

        PhysicsBodyTag(std::num::NonZeroUsize::new(1).unwrap())
    }

    fn drop(&mut self, body: PhysicsBodyTag){

    }
}