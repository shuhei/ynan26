extern crate failure;
extern crate reqwest;

pub mod config;
pub mod error;

pub use crate::config::Config;
pub use crate::error::{Error, ErrorKind, Result};
