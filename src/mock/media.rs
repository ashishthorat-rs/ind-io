use crate::error::DeviceError;
use crate::traits::{MediaControl, Sealed};

const MIN_CHANNEL: u32 = 1;
const MAX_VOLUME: u8 = 100;

#[derive(Debug, Clone)]
pub struct MockMedia {
    channel: u32,
    volume: u8,
    muted: bool,
}

impl MockMedia {
    pub fn new() -> Self {
        Self {
            channel: MIN_CHANNEL,
            volume: 0,
            muted: false,
        }
    }
}

impl Default for MockMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for MockMedia {}

impl MediaControl for MockMedia {
    fn channel_up(&mut self) -> Result<(), DeviceError> {
        self.channel = self.channel.saturating_add(1);
        Ok(())
    }

    fn channel_down(&mut self) -> Result<(), DeviceError> {
        if self.channel > MIN_CHANNEL {
            self.channel -= 1;
        }
        Ok(())
    }

    fn set_channel(&mut self, channel: u32) -> Result<(), DeviceError> {
        if channel < MIN_CHANNEL {
            return Err(DeviceError::InvalidInput);
        }
        self.channel = channel;
        Ok(())
    }

    fn set_volume(&mut self, volume: u8) -> Result<(), DeviceError> {
        if volume > MAX_VOLUME {
            return Err(DeviceError::InvalidInput);
        }
        self.volume = volume;
        Ok(())
    }

    fn volume_up(&mut self) -> Result<(), DeviceError> {
        if self.volume < MAX_VOLUME {
            self.volume += 1;
        }
        Ok(())
    }

    fn volume_down(&mut self) -> Result<(), DeviceError> {
        self.volume = self.volume.saturating_sub(1);
        Ok(())
    }

    fn mute(&mut self) -> Result<(), DeviceError> {
        self.muted = true;
        Ok(())
    }

    fn unmute(&mut self) -> Result<(), DeviceError> {
        self.muted = false;
        Ok(())
    }

    fn is_muted(&self) -> bool {
        self.muted
    }

    fn channel(&self) -> u32 {
        self.channel
    }

    fn volume(&self) -> u8 {
        self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_on_channel_one_silent_and_unmuted() {
        let mock = MockMedia::new();
        assert_eq!(mock.channel(), 1);
        assert_eq!(mock.volume(), 0);
        assert!(!mock.is_muted());
    }

    #[test]
    fn channel_up_and_down_step_by_one() {
        let mut mock = MockMedia::new();
        mock.channel_up().unwrap();
        mock.channel_up().unwrap();
        assert_eq!(mock.channel(), 3);
        mock.channel_down().unwrap();
        assert_eq!(mock.channel(), 2);
    }

    #[test]
    fn channel_down_reaches_and_stops_at_first_channel() {
        let mut mock = MockMedia::new();
        mock.channel_up().unwrap();
        mock.channel_down().unwrap();
        assert_eq!(mock.channel(), 1);
        mock.channel_down().unwrap();
        assert_eq!(mock.channel(), 1);
    }

    #[test]
    fn channel_up_does_not_overflow() {
        let mut mock = MockMedia::new();
        mock.set_channel(u32::MAX).unwrap();
        mock.channel_up().unwrap();
        assert_eq!(mock.channel(), u32::MAX);
    }

    #[test]
    fn set_channel_accepts_valid_and_rejects_zero() {
        let mut mock = MockMedia::new();
        mock.set_channel(47).unwrap();
        assert_eq!(mock.channel(), 47);
        assert_eq!(mock.set_channel(0), Err(DeviceError::InvalidInput));
        assert_eq!(mock.channel(), 47);
        mock.set_channel(1).unwrap();
        assert_eq!(mock.channel(), 1);
    }

    #[test]
    fn volume_up_and_down_step_by_one() {
        let mut mock = MockMedia::new();
        mock.volume_up().unwrap();
        mock.volume_up().unwrap();
        assert_eq!(mock.volume(), 2);
        mock.volume_down().unwrap();
        assert_eq!(mock.volume(), 1);
    }

    #[test]
    fn volume_stepping_reaches_and_stops_at_limits() {
        let mut mock = MockMedia::new();
        mock.set_volume(1).unwrap();
        mock.volume_down().unwrap();
        assert_eq!(mock.volume(), 0);
        mock.volume_down().unwrap();
        assert_eq!(mock.volume(), 0);

        mock.set_volume(99).unwrap();
        mock.volume_up().unwrap();
        assert_eq!(mock.volume(), 100);
        mock.volume_up().unwrap();
        assert_eq!(mock.volume(), 100);
    }

    #[test]
    fn set_volume_accepts_100_but_rejects_101() {
        let mut mock = MockMedia::new();
        mock.set_volume(100).unwrap();
        assert_eq!(mock.volume(), 100);
        assert_eq!(mock.set_volume(101), Err(DeviceError::InvalidInput));
        assert_eq!(mock.volume(), 100);
    }

    #[test]
    fn mute_and_unmute_are_idempotent_and_keep_volume() {
        let mut mock = MockMedia::new();
        mock.set_volume(30).unwrap();

        mock.mute().unwrap();
        mock.mute().unwrap();
        assert!(mock.is_muted());
        assert_eq!(mock.volume(), 30);

        mock.unmute().unwrap();
        mock.unmute().unwrap();
        assert!(!mock.is_muted());
        assert_eq!(mock.volume(), 30);
    }

    #[test]
    fn toggle_mute_flips_state_both_ways() {
        let mut mock = MockMedia::new();
        mock.toggle_mute().unwrap();
        assert!(mock.is_muted());
        mock.toggle_mute().unwrap();
        assert!(!mock.is_muted());
    }

    #[test]
    fn volume_changes_do_not_affect_mute_state() {
        let mut mock = MockMedia::new();
        mock.mute().unwrap();
        mock.set_volume(50).unwrap();
        mock.volume_up().unwrap();
        assert!(mock.is_muted());
        assert_eq!(mock.volume(), 51);
    }
}
