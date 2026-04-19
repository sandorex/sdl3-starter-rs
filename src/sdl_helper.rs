//! Contains replacement SDL3 main macro

use sdl3_sys::{events::SDL_Event, init::SDL_AppResult};
use sdl3_main::AppResult;

/// Trait for callback-driven SDL3 applications.
///
/// Implement this trait on your application struct and call [`run`] to start
/// the SDL3 main-callbacks loop. SDL will call your methods at the appropriate
/// times; you never write a manual event/render loop.
pub trait App: Sized {
    type Error;

    /// Called once at startup. Create your window, device, and resources here.
    /// Return `Err` to abort launch.
    fn init() -> Result<Self, Self::Error>;

    /// Called once per frame.
    fn iterate(&mut self) -> AppResult;

    /// Called once per pending event.
    fn event(&mut self, event: &SDL_Event) -> AppResult;

    /// Called once before the process exits. Clean up resources here if needed
    /// (though `Drop` impls will also run).
    fn quit(&mut self) {}
}

unsafe extern "C" fn app_init<T: App>(
    appstate: *mut *mut core::ffi::c_void,
    _argc: core::ffi::c_int,
    _argv: *mut *mut core::ffi::c_char,
) -> SDL_AppResult {
    match T::init() {
        Ok(app) => {
            let boxed = Box::new(app);
            unsafe { *appstate = Box::into_raw(boxed) as *mut core::ffi::c_void };
            SDL_AppResult::CONTINUE
        }
        Err(_) => SDL_AppResult::FAILURE,
    }
}

unsafe extern "C" fn app_iterate<T: App>(
    appstate: *mut core::ffi::c_void,
) -> SDL_AppResult {
    let app = unsafe { &mut *(appstate as *mut T) };

    app.iterate().into()
}

unsafe extern "C" fn app_event<T: App>(
    appstate: *mut core::ffi::c_void,
    event: *mut SDL_Event,
) -> SDL_AppResult {
    let app = unsafe { &mut *(appstate as *mut T) };

    app.event(unsafe {  &*event }).into()
}

unsafe extern "C" fn app_quit<T: App>(
    appstate: *mut core::ffi::c_void,
    _result: SDL_AppResult,
) {
    if !appstate.is_null() {
        let mut app = unsafe { Box::from_raw(appstate as *mut T) };
        app.quit();
        // Box is dropped here, running T's Drop impl
    }
}

/// Enter the SDL3 callback-based main loop with the given argc/argv.
/// Returns the process exit code. This is the raw entry point used by
/// [`run`] and [`sdl3_main!`].
///
/// # Safety
///
/// `argc` and `argv` must be valid C main arguments.
pub unsafe fn enter_main_callbacks<T: App>(
    argc: core::ffi::c_int,
    argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    unsafe {
        sdl3_sys::main::SDL_SetMainReady();
        sdl3_sys::main::SDL_EnterAppMainCallbacks(
            argc,
            argv,
            Some(app_init::<T>),
            Some(app_iterate::<T>),
            Some(app_event::<T>),
            Some(app_quit::<T>),
        )
    }
}

/// Enter the SDL3 callback-based main loop. This function never returns.
///
/// SDL will call `T::init`, then alternate between `T::event` and `T::iterate`
/// until one of them signals quit, at which point `T::quit` is called and the
/// process exits.
pub fn run<T: App>() -> ! {
    unsafe {
        // maybe this could be used?
        // let x = std::env::args_os().collect::<Vec<_>>();
        // x.as_mut_ptr()

        // TODO is the null_mut a valid argument?
        let rc = enter_main_callbacks::<T>(0, std::ptr::null_mut());
        std::process::exit(rc)
    }
}

/// Define an `SDL_main` entry point for the given [`App`] type.
///
/// On Android, the Java layer loads the shared library and calls `SDL_main`.
/// This macro exports the correctly-named, `#[no_mangle]` C symbol that
/// forwards to [`enter_main_callbacks`].
///
/// # Example
/// ```ignore
/// sdl3_gs::sdl3_main!(MyApp);
/// ```
#[macro_export]
macro_rules! sdl3_main {
    ($app:ty) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SDL_main(
            argc: ::core::ffi::c_int,
            argv: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int {
            unsafe { $crate::sdl_helper::enter_main_callbacks::<$app>(argc, argv) }
        }

        #[allow(unused)]
        fn main() {
            $crate::sdl_helper::run::<$app>();
        }
    };
}
