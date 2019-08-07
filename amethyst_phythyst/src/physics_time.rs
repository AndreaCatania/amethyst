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
pub struct PhysicsTime {
    /// The time used to advance the physics.
    /// The default is 60 frames per second : 1 / 60
    pub(crate) sub_step_seconds: f32,

    /// This is the maximum number of sub steps, to avoid spiral performance drop.
    /// Default is 8
    pub(crate) max_sub_steps: u32,

    /// ### IMPORTANT
    /// This is used internally, don't change it in any way please.
    pub(crate) _max_bank_size: f32,

    /// ### IMPORTANT
    /// This is used internally, don't change it in any way please.
    pub(crate) _time_bank: f32,
}

impl Default for PhysicsTime {
    fn default() -> Self {
        let t = PhysicsTime {
            sub_step_seconds: 0.0,
            max_sub_steps: 0,
            _max_bank_size: 0.0,
            _time_bank: 0.0,
        };
        t.set_frames_per_second(60).set_max_sub_steps(8)
    }
}

impl PhysicsTime {
    pub fn set_frames_per_second(mut self, frames_per_second: u32) -> Self {
        self.set_sub_step_seconds(1.0 / frames_per_second as f32)
    }

    pub fn set_max_sub_steps(mut self, max_sub_steps: u32) -> Self {
        self.max_sub_steps = max_sub_steps;
        self.update_max_bank_size();
        self
    }

    pub fn sub_step_seconds(&self) -> f32 {
        self.sub_step_seconds
    }

    pub fn sub_max_sub_steps(&self) -> u32 {
        self.max_sub_steps
    }

    fn set_sub_step_seconds(mut self, sub_step_seconds: f32) -> Self {
        self.sub_step_seconds = sub_step_seconds;
        self.update_max_bank_size();
        self
    }

    fn update_max_bank_size(&mut self) {
        self._max_bank_size = self.sub_step_seconds * self.max_sub_steps as f32;
    }
}
