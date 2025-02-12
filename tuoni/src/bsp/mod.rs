#[cfg(feature = "raspi4b")]
mod raspi4b;

#[cfg(not(feature = "raspi4b"))]
mod qemu;

#[cfg(feature = "raspi4b")]
pub use raspi4b::*;

#[cfg(not(feature = "raspi4b"))]
pub use qemu::*;
