use ind_io::DeviceError;

#[test]
fn display_text_for_each_variant() {
    let expected = [
        (DeviceError::HardwareFailure, "hardware failure"),
        (DeviceError::InvalidInput, "invalid input"),
        (DeviceError::NotSupported, "not supported"),
        (DeviceError::Timeout, "timed out"),
    ];

    for (error, text) in expected {
        assert_eq!(format!("{error}"), text);
    }
}

#[test]
fn display_respects_width_and_alignment() {
    assert_eq!(format!("{:>12}", DeviceError::Timeout), "   timed out");
}
