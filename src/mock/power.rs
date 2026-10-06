use crate::error::DeviceError;
use crate::traits::{PowerControl, Sealed};

#[derive(Debug, Default, Clone)]
pub struct MockPower {
    is_on: bool,
}

impl MockPower {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(is_on: bool) -> Self {
        Self { is_on }
    }
}

impl Sealed for MockPower {}

impl PowerControl for MockPower {
    fn turn_on(&mut self) -> Result<(), DeviceError> {
        self.is_on = true;
        Ok(())
    }

    fn turn_off(&mut self) -> Result<(), DeviceError> {
        self.is_on = false;
        Ok(())
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_off_by_default() {
        assert!(!MockPower::new().is_on());
    }

    #[test]
    fn with_state_sets_initial_state() {
        assert!(MockPower::with_state(true).is_on());
        assert!(!MockPower::with_state(false).is_on());
    }

    #[test]
    fn turn_on_then_off() {
        let mut mock = MockPower::new();
        mock.turn_on().unwrap();
        assert!(mock.is_on());
        mock.turn_off().unwrap();
        assert!(!mock.is_on());
    }

    #[test]
    fn toggle_flips_state_both_ways() {
        let mut mock = MockPower::new();
        mock.toggle().unwrap();
        assert!(mock.is_on());
        mock.toggle().unwrap();
        assert!(!mock.is_on());
    }

    #[test]
    fn reset_leaves_the_device_on_from_either_state() {
        let mut starting_off = MockPower::with_state(false);
        starting_off.reset().unwrap();
        assert!(starting_off.is_on());

        let mut starting_on = MockPower::with_state(true);
        starting_on.reset().unwrap();
        assert!(starting_on.is_on());
    }

    #[test]
    fn default_methods_work_through_a_trait_object() {
        let mut mock = MockPower::new();
        let power: &mut dyn PowerControl = &mut mock;
        power.toggle().unwrap();
        power.reset().unwrap();
        assert!(power.is_on());
    }
}
