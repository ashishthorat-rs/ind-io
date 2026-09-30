use crate::error::DeviceError;
use crate::traits::Sealed;

pub trait PositionControl: Sealed {
    fn set_position(&mut self, percent: u8) -> Result<(), DeviceError>;
    fn position(&self) -> u8;

    fn open(&mut self) -> Result<(), DeviceError> {
        self.set_position(100)
    }

    fn close(&mut self) -> Result<(), DeviceError> {
        self.set_position(0)
    }

    fn is_open(&self) -> bool {
        self.position() == 100
    }

    fn is_closed(&self) -> bool {
        self.position() == 0
    }
}
