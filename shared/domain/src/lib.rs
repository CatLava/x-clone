#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;


pub mod ids;

pub mod user;


pub use user::{Username, Password};

pub trait UserFacingError {
    fn formatted_error(&self) -> &'static str;
}