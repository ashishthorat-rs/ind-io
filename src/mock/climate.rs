use crate::error::DeviceError;
use crate::traits::{ClimateControl, Sealed};
use crate::types::{ClimateMode, SwingMode};

const DEFAULT_TEMPERATURE: f32 = 22.0;

#[derive(Debug, Clone)]
pub struct MockClimate {
    temperature: f32,
    mode: ClimateMode,
    swing: SwingMode,
}

impl MockClimate {
    pub fn new() -> Self {
        Self {
            temperature: DEFAULT_TEMPERATURE,
            mode: ClimateMode::Auto,
            swing: SwingMode::Off,
        }
    }
}

impl Default for MockClimate {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for MockClimate {}

impl ClimateControl for MockClimate {
    fn set_temperature(&mut self, celsius: f32) -> Result<(), DeviceError> {
        if !celsius.is_finite() {
            return Err(DeviceError::InvalidInput);
        }

        self.temperature = celsius;
        Ok(())
    }

    fn set_mode(&mut self, mode: ClimateMode) -> Result<(), DeviceError> {
        self.mode = mode;
        Ok(())
    }

    fn set_swing(&mut self, swing: SwingMode) -> Result<(), DeviceError> {
        self.swing = swing;
        Ok(())
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn mode(&self) -> ClimateMode {
        self.mode
    }

    fn swing(&self) -> SwingMode {
        self.swing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_22_celcius_in_auto_mode_with_swing_off() {
        let mock = MockClimate::new();
        assert_eq!(mock.temperature(), 22.0);
        assert_eq!(mock.mode(), ClimateMode::Auto);
        assert_eq!(mock.swing(), SwingMode::Off);
    }

    #[test]
    fn set_temperature_accepts_any_finite_value() {
        let mut mock = MockClimate::new();
        mock.set_temperature(7.0).unwrap();
        assert_eq!(mock.temperature(), 7.0);
        mock.set_temperature(-5.5).unwrap();
        assert_eq!(mock.temperature(), -5.5);
    }

    #[test]
    fn set_temperature_rejects_non_finite_values() {
        let mut mock = MockClimate::new();
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(mock.set_temperature(value), Err(DeviceError::InvalidInput));
            assert_eq!(mock.temperature(), 22.0);
        }
    }

    #[test]
    fn set_mode_updates_state() {
        let mut mock = MockClimate::new();
        for mode in [
            ClimateMode::Cool,
            ClimateMode::Heat,
            ClimateMode::Fan,
            ClimateMode::Dry,
            ClimateMode::Auto,
        ] {
            mock.set_mode(mode).unwrap();
            assert_eq!(mock.mode(), mode);
        }
    }

    #[test]
    fn set_swing_updates_state() {
        let mut mock = MockClimate::new();
        for swing in [
            SwingMode::Vertical,
            SwingMode::Horizontal,
            SwingMode::Both,
            SwingMode::Off,
        ] {
            mock.set_swing(swing).unwrap();
            assert_eq!(mock.swing(), swing);
        }
    }
}
