

use amethyst_phythyst::{
    servers::BodyServer,
    StoreTag,
};

pub struct NBodyServer{
}

impl NBodyServer{

    pub fn new() -> Self{
        NBodyServer{}
    }
}

impl BodyServer for NBodyServer{

    fn create(&mut self, world: StoreTag) -> StoreTag{

        StoreTag(std::num::NonZeroUsize::new(1).unwrap())
    }

    fn drop(&mut self, body: StoreTag){

    }
}