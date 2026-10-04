use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::MAX_PERCENT;
pub trait PositionControl: Sealed {
    fn set_position(&mut self, percent: u8) -> Result<(), DeviceError>;
    fn position(&self) -> u8;

    fn open(&mut self) -> Result<(), DeviceError> {
        self.set_position(MAX_PERCENT)
    }

    fn close(&mut self) -> Result<(), DeviceError> {
        self.set_position(0)
    }

    fn is_open(&self) -> bool {
        self.position() == MAX_PERCENT
    }

    fn is_closed(&self) -> bool {
        self.position() == 0
    }
}
