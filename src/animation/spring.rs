use glam::f32::Vec4;
use iced::time::Instant;

use super::{SecondOrderDynamics, settling_time_to_frequency};

#[derive(Debug, Clone)]
pub struct Spring<T> {
    target: T,
    value: T,
    dynamics: SecondOrderDynamics<Vec4>,
    updated: Instant,
    steady: bool,
}

impl<T> Spring<T>
where
    T: From<Vec4> + Clone,
    for<'a> &'a T: Into<Vec4>,
{
    pub fn new(f: f32, z: f32, r: f32, value: T) -> Self {
        Self {
            target: value.clone(),
            value: value.clone(),
            dynamics: SecondOrderDynamics::new(f, z, r, (&value).into()),
            updated: Instant::now(),
            steady: true,
        }
    }

    pub fn smooth(value: T) -> Self {
        let f = settling_time_to_frequency(0.4, 1.0);
        Self::new(f, 1.0, 0.0, value)
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn update(&mut self, now: Instant) {
        // How much time has passed since the last call to this function?
        let delta_time = now.saturating_duration_since(self.updated).as_secs_f32();
        self.updated = now;
        // Compute second order dynamics
        let input = (&self.target).into();
        let state = self.dynamics.update(delta_time, input);
        self.value = state.into();
        // Check if we reached steady state and stop the animation here.
        self.steady = input.abs_diff_eq(state, 0.0001)
            && self
                .dynamics
                .state_derivative()
                .abs_diff_eq(Vec4::ZERO, 0.0001);
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn is_animating(&self) -> bool {
        !self.steady
    }

    pub fn go(&mut self, target: T, now: Instant) {
        self.target = target;
        self.updated = now;
        self.steady = false;
    }
}

impl<T> Default for Spring<T>
where
    T: From<Vec4> + Clone + Default,
    for<'a> &'a T: Into<Vec4>,
{
    fn default() -> Self {
        Self::smooth(Default::default())
    }
}
