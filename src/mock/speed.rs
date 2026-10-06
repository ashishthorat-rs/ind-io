use crate::error::DeviceError;
use crate::traits::{Sealed, SpeedControl};
use crate::types::Speed;

#[derive(Debug, Clone)]
pub struct MockSpeed {
    speed: Speed,
}

impl MockSpeed {
    pub fn new() -> Self {
        Self { speed: Speed::Off }
    }
}

impl Default for MockSpeed {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for MockSpeed {}

impl SpeedControl for MockSpeed {
    fn set_speed(&mut self, speed: Speed) -> Result<(), DeviceError> {
        self.speed = speed;
        Ok(())
    }

    fn speed(&self) -> Speed {
        self.speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_off() {
        assert_eq!(MockSpeed::new().speed(), Speed::Off);
    }

    #[test]
    fn set_speed_updates_state() {
        let mut mock = MockSpeed::new();
        for speed in [
            Speed::Low,
            Speed::Medium,
            Speed::High,
            Speed::Turbo,
            Speed::Off,
        ] {
            mock.set_speed(speed).unwrap();
            assert_eq!(mock.speed(), speed);
        }
    }
}
