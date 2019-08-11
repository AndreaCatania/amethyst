use amethyst_core::{
    ecs::{
        BatchAccessor, BatchController, BatchUncheckedWorld, Dispatcher, Read, ReadExpect,
        System, WriteExpect, RunningTime, AccessorCow, World
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
    type BatchSystemData = (ReadExpect<'a, PhysicsTime>, ReadExpect<'a, Time>);

    unsafe fn create(accessor: BatchAccessor, dispatcher: Dispatcher<'a, 'b>) -> Self {
        PhysicsBatch {
            accessor,
            dispatcher,
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, N: PtReal> System<'a> for PhysicsBatch<'_, '_, N> {
    type SystemData = BatchUncheckedWorld<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let want_to_dispatch = {
            let time = data.0.fetch::<Time>();
            let mut physics_time = data.0.fetch_mut::<PhysicsTime>();

            physics_time._time_bank += time.delta_seconds();

            // Avoid spiral performance degradation
            physics_time._time_bank = physics_time._time_bank.min(physics_time._max_bank_size);

            physics_time._time_bank >= physics_time.sub_step_seconds
        };

        if want_to_dispatch {
            self.dispatcher.dispatch(data.0);
        }
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::VeryLong
    }

    fn accessor<'c>(&'c self) -> AccessorCow<'a, 'c, Self> {
        AccessorCow::Ref(&self.accessor)
    }

    fn setup(&mut self, world: &mut World) {
        self.dispatcher.setup(world);
    }
}

unsafe impl<N: PtReal> Send for PhysicsBatch<'_, '_, N>{}