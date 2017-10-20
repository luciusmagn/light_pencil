//! Pencil is a microframework for Rust inspired by [Flask](http://flask.pocoo.org/).
//!
//! # Installation
//!
//! This crate is called `pencil` and you can depend on it via cargo:
//!
//! ```ini
//! [dependencies]
//! pencil = "*"
//! ```
//!
//! # Quickstart
//!
//! A short introduction to Pencil.
//!
//! ## A Minimal Application
//!
//! A minimal Pencil application looks something like this:
//!
//! ```rust,no_run
//! extern crate sharp_pencil;
//!
//! use sharp_pencil::Pencil;
//! use sharp_pencil::{Request, PencilResult, Response};
//! use sharp_pencil::method::Get;
//!
//!
//! fn hello(_: &mut Request) -> PencilResult {
//!     Ok(Response::from("Hello World!"))
//! }
//!
//!
//! fn main() {
//!     let mut app = Pencil::new("/web/hello");
//!     app.route("/", &[Get], "hello", hello);
//!     app.run("127.0.0.1:5000");
//! }
//! ```

#![deny(non_camel_case_types)]

extern crate hyper;
extern crate regex;
extern crate url;
extern crate formdata;
extern crate typemap;
extern crate mime;
extern crate mime_guess;
extern crate lazycell;
extern crate time;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilError,
        PenHTTPError,
        PenUserError,
    UserError,
    PencilResult,
    ViewArgs,
    ViewFunc,
    UserErrorHandler,
    HTTPErrorHandler,
    BeforeRequestFunc,
    AfterRequestFunc,
    TeardownRequestFunc,
};
pub use wrappers::{
    Request,
    Response,
};
pub use http_errors::{
    HTTPError
};
pub use helpers::{
    PathBound,
    safe_join,
    abort,
    redirect,
    escape,
    send_file,
    send_from_directory,
};
pub use module::Module;

pub use hyper::header::{Cookie, SetCookie, Headers, ContentLength, ContentType};


#[macro_use]
mod utils;
pub mod http_errors;
pub mod datastructures;
pub mod wrappers;
pub mod routing;
pub mod helpers;
pub mod method;
mod app;
mod types;
mod serving;
mod httputils;
mod formparser;
mod module;
