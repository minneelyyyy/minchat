//! This library provided [`rocket_anyhow::Error`][Error],
//! a wrapper around [`anyhow::Error`]
//! with rocket's [responder] implemented.
//!
//! [anyhow::Error]: https://docs.rs/anyhow/1.0/anyhow/struct.Error.html
//! [responder]: https://api.rocket.rs/v0.4/rocket/response/trait.Responder.html
//!
//! ```no_run
//! # #[macro_use] extern crate rocket;
//! use std::io::Write;
//!
//! #[post("/<path..>", data="<text>")]
//! fn write_utf8_to(path: std::path::PathBuf, text: Vec<u8>) -> rocket_anyhow::Result {
//!    let mut file = std::fs::File::open(path)?;
//!    let text = std::str::from_utf8(&text)?;
//!    file.write_all(text.as_ref())?;
//!    Ok(())
//! }
//!
//! ```

use rocket::response::{self, Responder};
use rocket::Request;

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Wrapper around [`anyhow::Error`]
/// with rocket's [responder] implemented
///
/// [anyhow::Error]: https://docs.rs/anyhow/1.0/anyhow/struct.Error.html
/// [responder]: https://api.rocket.rs/v0.4/rocket/response/trait.Responder.html
/// Error that can be convert into `anyhow::Error` can be convert directly to this type.
///
/// Responder part are internally delegated to [rocket::response::Debug] which
/// "debug prints the internal value before responding with a 500 error"
///
/// [rocket::response::Debug]: https://api.rocket.rs/v0.4/rocket/response/struct.Debug.html
#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl<E> From<E> for crate::Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl<'a> Responder<'a, 'a> for Error {
    fn respond_to(self, request: &'a Request<'_>) -> response::Result<'a> {
        response::Debug(self.0).respond_to(request)
    }
}
