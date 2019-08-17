use nphysics3d::{
    object::BodySet as NpBodySet,
    force_generator::ForceGenerator as NpForceGenerator,
};
use amethyst_phythyst::{PtReal};

use crate::storage::StoreKey;

pub struct ForceGenerator<N: PtReal, S: NpBodySet<N>> {
    pub self_key: Option<StoreKey>,
    pub np_force_generator: Box<dyn NpForceGenerator<N, S>>,
    pub world_key: StoreKey,
}

impl<N: PtReal, S: NpBodySet<N>> ForceGenerator<N, S>{
    pub(crate) fn new(np_force_generator: Box<dyn NpForceGenerator<N, S>>, world_key: StoreKey) -> Self {
        ForceGenerator {
            self_key: None,
            np_force_generator,
            world_key,
        }
    }
}
