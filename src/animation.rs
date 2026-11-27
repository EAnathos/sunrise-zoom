//! Frame-time sampling for a reversible FOV transition.
use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct Transition {
    from: f32,
    target: f32,
    started: Instant,
    duration: Duration,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            from: 1.0,
            target: 1.0,
            started: Instant::now(),
            duration: Duration::ZERO,
        }
    }
}

impl Transition {
    /// Change direction from the currently displayed value, keeping rapid presses continuous.
    pub(crate) fn retarget(&mut self, target: f32, duration: Duration, now: Instant) {
        if (target - self.target).abs() < f32::EPSILON {
            return;
        }

        self.from = self.sample(now);
        self.target = target;
        self.started = now;
        self.duration = duration;
    }

    /// Sample a symmetric ease-in/ease-out cubic curve at the current render time.
    pub(crate) fn sample(&self, now: Instant) -> f32 {
        if self.duration.is_zero() {
            return self.target;
        }

        let elapsed = now.saturating_duration_since(self.started).as_secs_f32();
        let progress = (elapsed / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        let eased = if progress < 0.5 {
            4.0 * progress * progress * progress
        } else {
            1.0 - (-2.0 * progress + 2.0).powi(3) / 2.0
        };
        self.from + (self.target - self.from) * eased
    }
}
