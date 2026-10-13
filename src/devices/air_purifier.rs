use crate::error::DeviceError;
use crate::traits::{PowerControl, Sealed, SpeedControl};
use crate::types::Speed;

#[derive(Debug, Clone)]
pub struct AirPurifier<P, S> {
    power: P,
    speed: S,
}

impl<P: PowerControl, S: SpeedControl> AirPurifier<P, S> {
    pub fn new(power: P, speed: S) -> Self {
        Self { power, speed }
    }
}

impl<P, S> Sealed for AirPurifier<P, S> {}

impl<P: PowerControl, S> PowerControl for AirPurifier<P, S> {
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

impl<P, S: SpeedControl> SpeedControl for AirPurifier<P, S> {
    fn set_speed(&mut self, speed: Speed) -> Result<(), DeviceError> {
        self.speed.set_speed(speed)
    }

    fn speed(&self) -> Speed {
        self.speed.speed()
    }
}
