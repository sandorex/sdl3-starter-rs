//! Contains safe wrappers for some SDL3 functions
#![allow(unused)]

pub mod log;
pub mod panic_hook;
pub mod callback;
pub mod util;

pub use callback::App;
pub use panic_hook::setup_panic_hook;
pub use util::get_error;
