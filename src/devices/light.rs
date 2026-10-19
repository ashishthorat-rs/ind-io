use crate::error::DeviceError;
use crate::traits::{LightControl, PowerControl, Sealed};

#[derive(Debug, Clone)]
pub struct Light<P, L> {
    power: P,
    light: L,
}

impl<P: PowerControl, L: LightControl> Light<P, L> {
    pub fn new(power: P, light: L) -> Self {
        Self { power, light }
    }
}

impl<P, L> Sealed for Light<P, L> {}

impl<P: PowerControl, L> PowerControl for Light<P, L> {
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

impl<P, L: LightControl> LightControl for Light<P, L> {
    fn set_brightness(&mut self, percent: u8) -> Result<(), DeviceError> {
        self.light.set_brightness(percent)
    }

    fn brightness(&self) -> u8 {
        self.light.brightness()
    }
}
