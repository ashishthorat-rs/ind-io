use crate::error::DeviceError;
use crate::traits::{PowerControl, Sealed};

#[derive(Debug, Clone)]
pub struct Switch<P> {
    power: P,
}

impl<P: PowerControl> Switch<P> {
    pub fn new(power: P) -> Self {
        Self { power }
    }
}

impl<P> Sealed for Switch<P> {}

impl<P: PowerControl> PowerControl for Switch<P> {
    fn turn_on(&mut self) -> Result<(), DeviceError> {
        self.power.turn_on()
    }

    fn turn_off(&mut self) -> Result<(), DeviceError> {
        self.power.turn_off()
    }

    fn is_on(&self) -> bool {
        self.power.is_on()
    }
}
