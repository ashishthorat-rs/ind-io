use crate::error::DeviceError;
use crate::traits::{MediaControl, PowerControl, Sealed};

#[derive(Debug, Clone)]
pub struct Television<P, M> {
    power: P,
    media: M,
}

impl<P: PowerControl, M: MediaControl> Television<P, M> {
    pub fn new(power: P, media: M) -> Self {
        Self { power, media }
    }
}

impl<P, M> Sealed for Television<P, M> {}

impl<P: PowerControl, M> PowerControl for Television<P, M> {
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

impl<P, M: MediaControl> MediaControl for Television<P, M> {
    fn channel_up(&mut self) -> Result<(), DeviceError> {
        self.media.channel_up()
    }
    fn channel_down(&mut self) -> Result<(), DeviceError> {
        self.media.channel_down()
    }

    fn set_channel(&mut self, channel: u32) -> Result<(), DeviceError> {
        self.media.set_channel(channel)
    }

    fn set_volume(&mut self, volume: u8) -> Result<(), DeviceError> {
        self.media.set_volume(volume)
    }

    fn volume_up(&mut self) -> Result<(), DeviceError> {
        self.media.volume_up()
    }

    fn volume_down(&mut self) -> Result<(), DeviceError> {
        self.media.volume_down()
    }

    fn mute(&mut self) -> Result<(), DeviceError> {
        self.media.mute()
    }

    fn unmute(&mut self) -> Result<(), DeviceError> {
        self.media.unmute()
    }

    fn is_muted(&self) -> bool {
        self.media.is_muted()
    }

    fn channel(&self) -> u32 {
        self.media.channel()
    }

    fn volume(&self) -> u8 {
        self.media.volume()
    }
}
