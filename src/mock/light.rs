use crate::error::DeviceError;
use crate::traits::{LightControl, Sealed};
use crate::types::MAX_PERCENT;

#[derive(Debug, Clone)]
pub struct MockLight {
    brightness: u8,
}

impl MockLight {
    pub fn new() -> Self {
        Self { brightness: 0 }
    }
}

impl Default for MockLight {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for MockLight {}

impl LightControl for MockLight {
    fn set_brightness(&mut self, percent: u8) -> Result<(), DeviceError> {
        if percent > MAX_PERCENT {
            return Err(DeviceError::InvalidInput);
        }

        self.brightness = percent;
        Ok(())
    }

    fn brightness(&self) -> u8 {
        self.brightness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_zero_brightness() {
        assert_eq!(MockLight::new().brightness, 0);
    }

    #[test]
    fn set_brightness_updates_state() {
        let mut mock = MockLight::new();
        for percent in [50, 100, 0] {
            mock.set_brightness(percent).unwrap();
            assert_eq!(mock.brightness(), percent);
        }
    }

    #[test]
    fn set_brightness_rejects_values_above_100() {
        let mut mock = MockLight::new();
        mock.set_brightness(40).unwrap();
        assert_eq!(mock.set_brightness(101), Err(DeviceError::InvalidInput));
        assert_eq!(mock.set_brightness(255), Err(DeviceError::InvalidInput));
        assert_eq!(mock.brightness(), 40);
    }
}
