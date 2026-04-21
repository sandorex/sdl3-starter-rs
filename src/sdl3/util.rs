//! Misc SDL stuff that dont have their own category

use std::ffi::{CStr, CString};
use sdl3_sys::error::SDL_GetError;

/// Gets SDL error and copies it
pub fn get_error() -> String {
    let cstr = unsafe {
        let raw_err = SDL_GetError();
        if raw_err.is_null() {
            return "".to_string();
        }

        CStr::from_ptr(raw_err)
    };

    cstr.to_string_lossy().to_string()
}
