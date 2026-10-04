use crate::error::DeviceError;
use crate::traits::Sealed;

pub trait PowerControl: Sealed {
    fn turn_on(&mut self) -> Result<(), DeviceError>;
    fn turn_off(&mut self) -> Result<(), DeviceError>;
    fn is_on(&self) -> bool;

    fn toggle(&mut self) -> Result<(), DeviceError> {
        if self.is_on() {
            self.turn_off()
        } else {
            self.turn_on()
        }
    }

    fn reset(&mut self) -> Result<(), DeviceError> {
        self.turn_off()?;
        self.turn_on()
    }
}
