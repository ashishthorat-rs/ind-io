use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::{ClimateMode, SwingMode};

pub trait ClimateControl: Sealed {
    fn set_temperature(&mut self, celsius: f32) -> Result<(), DeviceError>;
    fn set_mode(&mut self, mode: ClimateMode) -> Result<(), DeviceError>;
    fn set_swing(&mut self, swing: SwingMode) -> Result<(), DeviceError>;
    fn temperature(&self) -> f32;
    fn mode(&self) -> ClimateMode;
    fn swing(&self) -> SwingMode;
}
