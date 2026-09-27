// TODO: add implementation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceError {
    HardwareFailure,
    InvalidInput,
    NotSupported,
    Timeout,
    NotInitialized,
}
