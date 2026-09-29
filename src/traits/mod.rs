mod private {
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

pub mod climate;
pub mod light;
pub mod media;
pub mod position;
pub mod power;
pub mod speed;

pub use climate::ClimateControl;
