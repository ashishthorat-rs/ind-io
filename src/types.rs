// TODO: add implementation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    Off,
    Low,
    Medium,
    High,
    Turbo,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClimateMode {
    Cool,
    Heat,
    Dry,
    Fan,
    Auto,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingMode {
    Off,
    Vertical,
    Horizontal,
    Both,
}
