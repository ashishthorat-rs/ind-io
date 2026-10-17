use crate::error::DeviceError;
use crate::traits::{ClimateControl, PowerControl, Sealed};
use crate::types::{ClimateMode, SwingMode};

#[derive(Debug, Clone)]
pub struct Heater<P, C> {
    power: P,
    climate: C,
}

impl<P: PowerControl, C: ClimateControl> Heater<P, C> {
    pub fn new(power: P, climate: C) -> Self {
        Self { power, climate }
    }
}

impl<P, C> Sealed for Heater<P, C> {}

impl<P: PowerControl, C> PowerControl for Heater<P, C> {
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

impl<P, C: ClimateControl> ClimateControl for Heater<P, C> {
    fn set_temperature(&mut self, celsius: f32) -> Result<(), DeviceError> {
        self.climate.set_temperature(celsius)
    }

    fn set_mode(&mut self, mode: ClimateMode) -> Result<(), DeviceError> {
        self.climate.set_mode(mode)
    }

    fn set_swing(&mut self, swing: SwingMode) -> Result<(), DeviceError> {
        self.climate.set_swing(swing)
    }

    fn temperature(&self) -> f32 {
        self.climate.temperature()
    }

    fn mode(&self) -> ClimateMode {
        self.climate.mode()
    }

    fn swing(&self) -> SwingMode {
        self.climate.swing()
    }
}
