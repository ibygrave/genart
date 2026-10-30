#![allow(clippy::missing_errors_doc)]
mod boids;
mod colour;
mod field;
mod kayconfig;
mod region;

pub use boids::Flock;
pub use colour::{Colour, Colours};
pub use field::{Field, FieldArgs};
pub use kayconfig::KayConfig;
pub use region::Region;
