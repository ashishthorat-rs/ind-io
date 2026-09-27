// TODO: add implementation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    HardwareFailure,
    InvalidInput,
    NotSupported,
    Timeout,
    NotInitialized,
}
