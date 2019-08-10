use amethyst_core::{
    ecs::{
        BatchAccessor, BatchController, BatchUncheckedWorld, Dispatcher, Read, ReadExpect, RunNow,
        System, WriteExpect,
    },
    shred::Resources,
    Time,
};

use crate::{objects::*, servers::WorldPhysicsServer, PhysicsTime, PtReal};

pub struct PhysicsBatch<'a, 'b, N: crate::PtReal> {
    accessor: BatchAccessor,
    dispatcher: Dispatcher<'a, 'b>,
    phantom_data: std::marker::PhantomData<N>,
}

impl<'a, 'b, N: PtReal> BatchController<'a, 'b> for PhysicsBatch<'a, 'b, N> {
    type BatchSystemData = ();

    unsafe fn create(accessor: BatchAccessor, dispatcher: Dispatcher<'a, 'b>) -> Self {
        PhysicsBatch {
            accessor,
            dispatcher,
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, 'b, N: PtReal> System<'a> for PhysicsBatch<'_, '_, N> {
    type SystemData = BatchUncheckedWorld<'a>;

    fn run(&mut self, data: Self::SystemData) {}
}

unsafe impl<'a, 'b, N: PtReal> Send for PhysicsBatch<'a, 'b, N>{}