use crate::error::DeviceError;
use crate::traits::{ClimateControl, PowerControl, Sealed, SpeedControl};
use crate::types::{ClimateMode, Speed, SwingMode};

#[derive(Debug, Clone)]
pub struct AirConditioner<P, S, C> {
    power: P,
    speed: S,
    climate: C,
}

impl<P: PowerControl, S: SpeedControl, C: ClimateControl> AirConditioner<P, S, C> {
    pub fn new(power: P, speed: S, climate: C) -> Self {
        Self {
            power,
            speed,
            climate,
        }
    }
}

impl<P, S, C> Sealed for AirConditioner<P, S, C> {}

impl<P: PowerControl, S, C> PowerControl for AirConditioner<P, S, C> {
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

impl<P, S: SpeedControl, C> SpeedControl for AirConditioner<P, S, C> {
    fn set_speed(&mut self, speed: Speed) -> Result<(), DeviceError> {
        self.speed.set_speed(speed)
    }

    fn speed(&self) -> Speed {
        self.speed.speed()
    }
}

impl<P, S, C: ClimateControl> ClimateControl for AirConditioner<P, S, C> {
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
