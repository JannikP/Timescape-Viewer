use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};

// TODO: Document this and link to t3ssel8r's youtube video.
#[derive(Debug, Clone)]
pub struct SecondOrderDynamics<T> {
    previous_input: T,
    state: T,
    state_derivative: T,

    /// Omega
    w: f32,
    z: f32,
    d: f32,
    k1: f32,
    k2: f32,
    k3: f32,
}

impl<T> SecondOrderDynamics<T>
where
    T: Copy + Default + Add<Output = T> + Mul<f32, Output = T> + Sub<Output = T>,
{
    pub fn new(frequency: f32, dampening: f32, r: f32, x0: T) -> Self {
        let w = 2.0 * PI * frequency;
        let d = w * (dampening * dampening - 1.0).abs().sqrt();
        let k1 = dampening / (PI * frequency);
        let k2 = 1.0 / (w * w);
        let k3 = r * 2.0 / w;
        Self {
            previous_input: x0,
            state: x0,
            state_derivative: Default::default(),
            w,
            z: dampening,
            d,
            k1,
            k2,
            k3,
        }
    }

    pub fn update(&mut self, delta_time: f32, input: T) -> T {
        if delta_time > 0.0 {
            // Estimate velocity
            let input_derivative = (input - self.previous_input) * (1.0 / delta_time);
            self.previous_input = input;
            let (k1_stable, k2_stable) = if self.w * delta_time < self.z {
                // Clamp k2 to guarantee stability without jitter
                let k2_limit_1 = delta_time * delta_time * 0.5 + delta_time * self.k1 * 0.5;
                let k2_limit_2 = delta_time * self.k1;
                (self.k1, self.k2.max(k2_limit_1.max(k2_limit_2)))
            } else {
                // Use pole matching when the system is very fast
                let t1 = (-1.0 * self.z * self.w * delta_time).exp();
                let td = delta_time * self.d;
                let tdc = if self.z <= 1.0 { td.cos() } else { td.cosh() };
                let alpha = 2.0 * t1 * tdc;
                let beta = t1 * t1;
                let t2 = delta_time / (1.0 + beta - alpha);
                (((1.0 - beta) * t2), (delta_time * t2))
            };
            // Integrate position by velocity
            self.state = self.state + self.state_derivative * delta_time;
            // Integrate velocity by acceleration
            self.state_derivative = self.state_derivative
                + (input + input_derivative * self.k3
                    - self.state
                    - self.state_derivative * k1_stable)
                    * (delta_time / k2_stable);
        }
        // Return state (y)
        self.state
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn state_derivative(&self) -> &T {
        &self.state_derivative
    }
}

pub fn settling_time_to_frequency(settling_time: f32, damping: f32) -> f32 {
    //    t_settling = 5 / (w_n * z)
    // => w_n = 5 / (t_settling * z)
    5.0 / (settling_time * damping * 2.0 * PI)
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f32_near;

    use super::*;

    #[test]
    fn settles_at_critical_damping() {
        // Arrange
        let frequency = settling_time_to_frequency(0.4, 1.0);
        let mut system = SecondOrderDynamics::new(frequency, 1.0, 0.0, 0.0);
        let delta_time = 1.0 / 60.0; // 60 FPS
        let deadline = 6.0 * 5.0 / (frequency * 1.0 * 2.0 * PI);
        let mut time = 0.0;
        let mut value = 0.0;

        // Act
        while time < deadline {
            value = system.update(delta_time, 1.0);
            time += delta_time;
        }

        // Assert
        assert_f32_near!(value, 1.0);
        assert!(system.state_derivative().abs() < 0.00001);
    }

    #[test]
    fn settles_at_bouncy_settings() {
        // Arrange
        let frequency = settling_time_to_frequency(0.4, 1.0);
        let mut system = SecondOrderDynamics::new(frequency, 0.5, 2.0, 0.0);
        let delta_time = 1.0 / 60.0; // 60 FPS
        let deadline = 6.0 * 5.0 / (frequency * 1.0 * 2.0 * PI);
        let mut time = 0.0;
        let mut value = 0.0;

        // Act
        while time < deadline {
            value = system.update(delta_time, 1.0);
            time += delta_time;
        }

        // Assert
        assert_f32_near!(value, 1.0);
        assert!(system.state_derivative().abs() < 0.00001);
    }
}
