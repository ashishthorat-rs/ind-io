use core::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    HardwareFailure,
    InvalidInput,
    NotSupported,
    Timeout,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            DeviceError::HardwareFailure => "hardware failure",
            DeviceError::InvalidInput => "invalid input",
            DeviceError::NotSupported => "not supported",
            DeviceError::Timeout => "timed out",
        };
        f.pad(message)
    }
}
