
/// This resource is used to tweak the physics time step and at the same time store important
/// information used by the stepper system.
///
/// # Settings
/// To change the time step and the maximum steps executed per frame you can tweak the following
/// parameters, respectively, `sub_step_seconds` and `max_sub_steps`.
///
/// # Sub step dispatcher
/// Is possible to execute some system each sub step synchronization, to understand why and how
/// check this doc: TODO please put the link to where is explained how to used the sub step dispatcher
///
/// # Advanced
/// The physics stepping is performed in the system `PhysicsStepperSystem`.
///
/// This system takes the total frame time and break it in sub steps depending on the `sub_step_seconds`.
/// This is done because to have a stable simulation you want to step the process at fixed rate.
///
/// Depending on the `sub_step_seconds` and, on the `Timer::delta_seconds` could be necessary run the
/// stepping multiple times on the same frame.
///
/// Sometime could happen that the `Timer::delta_time` is so big that too much stepping are performed
/// in a single frame increasing even more the `Timer::delta_time` of the next frame, entering so
/// in a spiral that will drop the performances.
/// For this reason the `max_sub_steps` is necessary in order to counter this behavior.
pub struct PhysicsTime{

    /// The time used to advance the physics.
    /// The default is 60 frames per second : 1 / 60
    pub sub_step_seconds: f32,

    /// This is the maximum number of sub steps, to avoid spiral performance drop.
    /// Default is 8
    pub max_sub_steps: u8,

    /// ### IMPORTANT
    /// This is used internally, don't change it in any way please.
    pub _time_bank: f32,
}

impl Default for PhysicsTime{
    fn default() -> Self{
        PhysicsTime{
            sub_step_seconds: 1.0 / 60.0,
            max_sub_steps: 8,
            _time_bank: 0.0,
        }
    }
}