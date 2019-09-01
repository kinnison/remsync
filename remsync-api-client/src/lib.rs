//! API client functionality for remsync

pub(crate) mod util;

pub mod ll;

/// Generic error used because I'm too lazy to make a good one
type GenericError = std::boxed::Box<dyn std::error::Error + Send + Sync>;

/// Generic result used because I'm too lazy to make a good one
type GenericResult<T> = std::result::Result<T, GenericError>;

// Reexport hyper and http here to ensure that we always use the same
// since for now we're getting it via git
pub use http;
pub use hyper;
