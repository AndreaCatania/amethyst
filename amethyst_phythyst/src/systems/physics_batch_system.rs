use amethyst_core::{
    ecs::{
        AccessorCow, BatchAccessor, BatchController, BatchUncheckedWorld, Dispatcher, Read,
        ReadExpect, RunningTime, System, World, WriteExpect,
    },
    shred::Resources,
    Time,
};

use crate::{objects::*, PhysicsTime, PtReal};

/// This `Batch` is a used to dispatch the physics `System`s.
///
/// Depending on the `PhysicsTime::_time_bank` value, could be necessary run the stepping, multiple
/// times on the same frame.
///
///
/// Each frame, to the `_time_bank` is added the frame _delta time_ (which is variable).
///
/// Sometimes, could happens that the `Timer::delta_time` is so big that too much sub steps have to
/// be processed in order to consume the `_time_bank`.
/// This process, will increase the delta time of the next frame, entering so
/// in a spiral that will drop the performances.
///
/// To break this behavior a fall back algorithm, will clamp the maximum size of the `_time_bank`.
///
/// You can control the maximum `_time_bank` by changing the `max_sub_steps`.
pub struct PhysicsBatchSystem<'a, 'b, N: crate::PtReal> {
    accessor: BatchAccessor,
    dispatcher: Dispatcher<'a, 'b>,
    phantom_data: std::marker::PhantomData<N>,
}

impl<'a, 'b, N: PtReal> BatchController<'a, 'b> for PhysicsBatchSystem<'a, 'b, N> {
    type BatchSystemData = (ReadExpect<'a, PhysicsTime>, ReadExpect<'a, Time>);

    unsafe fn create(accessor: BatchAccessor, dispatcher: Dispatcher<'a, 'b>) -> Self {
        PhysicsBatchSystem {
            accessor,
            dispatcher,
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, N: PtReal> System<'a> for PhysicsBatchSystem<'_, '_, N> {
    type SystemData = BatchUncheckedWorld<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let want_to_dispatch = {
            let time = data.0.fetch::<Time>();
            let mut physics_time = data.0.fetch_mut::<PhysicsTime>();

            physics_time._time_bank += time.delta_seconds();

            // Avoid spiral performance degradation
            physics_time._time_bank = physics_time._time_bank.min(physics_time._max_bank_size);

            physics_time._time_bank >= physics_time.delta_seconds
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

unsafe impl<N: PtReal> Send for PhysicsBatchSystem<'_, '_, N> {}
