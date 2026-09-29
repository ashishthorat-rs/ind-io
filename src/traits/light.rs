use crate::error::DeviceError;
use crate::traits::Sealed;

pub trait LightControl: Sealed {
    fn set_brightness(&mut self, percent: u8) -> Result<(), DeviceError>;
    fn brightness(&self) -> u8;
}
