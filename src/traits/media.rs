use crate::error::DeviceError;
use crate::traits::Sealed;

pub trait MediaControl: Sealed {
    fn channel_up(&mut self) -> Result<(), DeviceError>;
    fn channel_down(&mut self) -> Result<(), DeviceError>;
    fn set_channel(&mut self, channel: u32) -> Result<(), DeviceError>;
    fn set_volume(&mut self, volume: u8) -> Result<(), DeviceError>;
    fn volume_up(&mut self) -> Result<(), DeviceError>;
    fn volume_down(&mut self) -> Result<(), DeviceError>;
    fn mute(&mut self) -> Result<(), DeviceError>;
    fn unmute(&mut self) -> Result<(), DeviceError>;
    fn is_muted(&self) -> bool;
    fn channel(&self) -> u32;
    fn volume(&self) -> u8;

    fn toggle_mute(&mut self) -> Result<(), DeviceError> {
        if self.is_muted() {
            self.unmute()
        } else {
            self.mute()
        }
    }
}
