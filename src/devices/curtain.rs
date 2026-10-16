use crate::error::DeviceError;
use crate::traits::{PositionControl, PowerControl, Sealed};

#[derive(Debug, Clone)]
pub struct Curtain<P, M> {
    power: P,
    position: M,
}

impl<P: PowerControl, M: PositionControl> Curtain<P, M> {
    pub fn new(power: P, position: M) -> Self {
        Self { power, position }
    }
}

impl<P, M> Sealed for Curtain<P, M> {}

impl<P: PowerControl, M> PowerControl for Curtain<P, M> {
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

impl<P, M: PositionControl> PositionControl for Curtain<P, M> {
    fn set_position(&mut self, percent: u8) -> Result<(), DeviceError> {
        self.position.set_position(percent)
    }

    fn position(&self) -> u8 {
        self.position.position()
    }
}
