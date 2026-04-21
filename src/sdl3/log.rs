//! Contains code to wrap SDL3 Log functions

// NOTE: assumes there wont be a message with a \0 NUL character!
#[macro_export]
macro_rules! log_ {
    ($fn:path, $msg:literal $(, $arg:tt)*) => {
        {
            // NOTE i am intentionally adding a NUL at the end so its a valid cstring
            let string = format!(concat!($msg, "\0") $(, $arg)*);

            // SAFETY: it is safe as the NUL character has to be there
            let cstr = std::ffi::CStr::from_bytes_until_nul(string.as_bytes()).unwrap();

            // call the wanted SDL log function
            unsafe { $fn(sdl3_sys::log::SDL_LOG_CATEGORY_APPLICATION.into(), cstr.as_ptr()) };
        }
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogTrace, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_verbose {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogVerbose, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_debug {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogDebug, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_info {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogInfo, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_warn {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogWarn, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_error {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogError, $msg $(, $arg)*)
    }
}

#[macro_export]
macro_rules! log_critical {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_!(sdl3_sys::log::SDL_LogCritical, $msg $(, $arg)*)
    }
}
