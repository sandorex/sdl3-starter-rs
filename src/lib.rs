pub mod ssl_cert;
pub mod sdl3;

use std::{ffi::{CStr, CString, OsStr}, path::{Path, PathBuf}, ptr::null, str::FromStr};
use git2::{CertificateCheckStatus, build::RepoBuilder};
use sdl3_sys::{everything::{SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_GetError, SDL_INIT_VIDEO, SDL_Init, SDL_Renderer, SDL_SetAppMetadata, SDL_Window, SDL_WindowFlags}, filesystem::{SDL_Folder, SDL_GetUserFolder}, log::SDL_LOG_CATEGORY_SYSTEM};
use sdl3_main::AppResult;
use sdl3_sys::events::SDL_Event;

pub struct MyAppState {
    pub window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    pub cloned: bool,
}

impl Drop for MyAppState {
    fn drop(&mut self) {
        unsafe {
            if !self.renderer.is_null() {
                SDL_DestroyRenderer(self.renderer);
            }

            if !self.window.is_null() {
                SDL_DestroyWindow(self.window);
            }
        }
    }
}

impl sdl3::App for MyAppState {
    type Error = ();

    fn init() -> Result<Self, Self::Error> {
        let title = c"Heeyy";

        unsafe {
            SDL_SetAppMetadata(c"SDL3 Starter Rust".as_ptr(), c"1.0".as_ptr(), c"com.example.sdl3-starter-rs".as_ptr());

            if !SDL_Init(SDL_INIT_VIDEO) {
                sdl_error_log("SDL_Init failed");
                return Err(());
            }

            let window = SDL_CreateWindow(title.as_ptr(), 640, 480, SDL_WindowFlags::RESIZABLE);
            if window.is_null() {
                sdl_error_log("SDL_CreateWindow failed");
                return Err(());
            }

            let renderer = SDL_CreateRenderer(window, null());
            if renderer.is_null() {
                sdl_error_log("SDL_CreateRenderer failed");
                return Err(());
            }

            use sdl3_sys::everything::{SDL_SetRenderDrawColor, SDL_RenderClear, SDL_RenderPresent};

            // render some color
            SDL_SetRenderDrawColor(renderer, 128, 128, 0, 255); // yellow
            SDL_RenderClear(renderer);
            SDL_RenderPresent(renderer);

            return Ok(MyAppState {
                window,
                renderer,
                cloned: false,
            })
        }
    }

    fn iterate(&mut self) -> AppResult {
        AppResult::Continue
    }

    fn event(&mut self, event: &SDL_Event) -> AppResult {
        use sdl3_sys::events::{SDL_EVENT_QUIT, SDL_EVENT_KEY_UP, SDL_EVENT_KEY_DOWN, SDL_EVENT_WINDOW_RESIZED};

        match event.event_type() {
            SDL_EVENT_QUIT => return AppResult::Success,
            SDL_EVENT_WINDOW_RESIZED => unsafe { println!("resize {:?}x{:?}", event.window.data1, event.window.data2) },
            _ => {},
        }

        AppResult::Continue
    }
}

pub fn sdl_error_log(msg: &str) {
    let err = sdl3::get_error();
    log_critical!("{}: {}", msg, err);
}

// create the required functions
sdl3_main!(MyAppState);
