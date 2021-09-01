#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate log;

mod binder;
pub use binder::*;

mod parcel;
pub use parcel::*;

mod parcelable;
pub use parcelable::*;

mod service;
pub use service::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("deserialization error")]
    DeserializationError,
    #[error("bad enum value")]
    BadEnumValue,
}
