use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::Speed;

pub trait SpeedControl: Sealed {
    fn set_speed(&mut self, speed: Speed) -> Result<(), DeviceError>;
    fn speed(&self) -> Speed;
}
