/// This resource is used by `Phythyst` to keep track of the physics time.
///
/// You can use it to know the actual physics delta time, or to change the physics frame per second.
///
/// # Max sub steps
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
#[derive(Debug)]
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
        t.with_frames_per_second(60).with_max_sub_steps(8)
    }
}

impl PhysicsTime {
    /// Set the physics frames per seconds.
    pub fn with_frames_per_second(mut self, frames_per_seconds: u32) -> Self {
        self.set_frames_per_seconds(frames_per_seconds);
        self
    }

    /// Set the physics frames per seconds.
    pub fn set_frames_per_seconds(&mut self, frames_per_seconds: u32) {
        self.set_sub_step_seconds(1.0 / frames_per_seconds as f32);
    }

    /// Set the physics max sub steps.
    /// This controls how much physics step can be executed in a single frame. It's used to avoid
    /// spiral performance degradation.
    /// Set it to an too high value, will make this mechanism ineffective, and a too low value will make the physics unstable.
    /// Is advised to keep the default
    pub fn with_max_sub_steps(mut self, max_sub_steps: u32) -> Self {
        self.set_max_sub_steps(max_sub_steps);
        self
    }

    /// Set the physics max sub steps.
    /// This controls how much physics step can be executed in a single frame. It's used to avoid
    /// spiral performance degradation.
    /// Set it to an too high value, will make this mechanism ineffective, and a too low value will make the physics unstable.
    /// Is advised to keep the default
    pub fn set_max_sub_steps(&mut self, max_sub_steps: u32) {
        self.max_sub_steps = max_sub_steps;
        self.update_max_bank_size();
    }

    /// Returns the max sub steps
    pub fn sub_max_sub_steps(&self) -> u32 {
        self.max_sub_steps
    }

    /// Set the sub step seconds, this function is used internally.
    fn set_sub_step_seconds(&mut self, sub_step_seconds: f32) {
        self.sub_step_seconds = sub_step_seconds;
        self.update_max_bank_size();
    }

    /// Get the seb step seconds
    pub fn sub_step_seconds(&self) -> f32 {
        self.sub_step_seconds
    }

    /// Updates the max bank size according to the actual frame per second and the allowed sub steps.
    /// This function is used internally.
    fn update_max_bank_size(&mut self) {
        self._max_bank_size = self.sub_step_seconds * self.max_sub_steps as f32;
    }
}
