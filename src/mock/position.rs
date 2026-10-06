use crate::error::DeviceError;
use crate::traits::{PositionControl, Sealed};
use crate::types::MAX_PERCENT;

#[derive(Debug, Clone)]
pub struct MockPosition {
    position: u8,
}

impl MockPosition {
    pub fn new() -> Self {
        Self { position: 0 }
    }
}

impl Default for MockPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl Sealed for MockPosition {}

impl PositionControl for MockPosition {
    fn set_position(&mut self, percent: u8) -> Result<(), DeviceError> {
        if percent > MAX_PERCENT {
            return Err(DeviceError::InvalidInput);
        }
        self.position = percent;
        Ok(())
    }

    fn position(&self) -> u8 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_closed() {
        let mock = MockPosition::new();
        assert_eq!(mock.position(), 0);
        assert!(mock.is_closed());
        assert!(!mock.is_open());
    }

    #[test]
    fn set_position_updates_state() {
        let mut mock = MockPosition::new();
        for percent in [50, 100, 0] {
            mock.set_position(percent).unwrap();
            assert_eq!(mock.position(), percent);
        }
    }

    #[test]
    fn set_position_rejects_values_above_100() {
        let mut mock = MockPosition::new();
        mock.set_position(40).unwrap();
        assert_eq!(mock.set_position(101), Err(DeviceError::InvalidInput));
        assert_eq!(mock.position(), 40);
    }

    #[test]
    fn open_and_close_reach_the_limits() {
        let mut mock = MockPosition::new();
        mock.open().unwrap();
        assert_eq!(mock.position(), 100);
        assert!(mock.is_open());
        mock.close().unwrap();
        assert_eq!(mock.position(), 0);
        assert!(mock.is_closed());
    }

    #[test]
    fn partial_position_is_neither_open_nor_closed() {
        let mut mock = MockPosition::new();
        mock.set_position(99).unwrap();
        assert!(!mock.is_open());
        assert!(!mock.is_closed());
    }
}
