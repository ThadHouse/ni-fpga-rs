#![cfg_attr(feature = "use_generic_const_exprs", feature(generic_const_exprs))]

extern crate ni_fpga_sys as ffi;

mod datatype;
mod erased_register;
mod errors;
mod fixed_register;
pub mod fxp;
mod hmb;
mod nifpga;
mod register;
mod session;
pub mod session_lifetimes;
mod status;

// Keep these for backwards compatibility, but don't use them internally
pub use datatype::{Datatype, FpgaBits};
pub use errors::Error;
pub type Offset = ffi::Offset;
pub use fixed_register::FixedRegisterAccess;
pub use hmb::Hmb;
pub use hmb::HmbAccess;
pub use register::Register;
#[cfg(feature = "use_generic_const_exprs")]
pub use register::RegisterAccess;
pub use session::Session;
pub use session::SessionAccess;
pub use status::Status;
