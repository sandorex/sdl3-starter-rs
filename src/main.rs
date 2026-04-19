mod sdl_helper;

use std::{ffi::CStr, ptr::null};
use sdl3_sys::everything::{SDL_Init, SDL_INIT_VIDEO, SDL_CreateWindow, SDL_Renderer, SDL_Window, SDL_DestroyWindow, SDL_DestroyRenderer, SDL_WindowFlags, SDL_CreateRenderer, SDL_SetAppMetadata, SDL_GetError};
use sdl3_main::AppResult;
use sdl3_sys::events::SDL_Event;

struct MyAppState {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
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

impl sdl_helper::App for MyAppState {
    type Error = ();

    fn init() -> Result<Self, Self::Error> {
        let title = c"Heeyy";

        unsafe {
            SDL_SetAppMetadata(c"SDL3 Starter Rust".as_ptr(), c"1.0".as_ptr(), c"com.example.sdl3-starter-rs".as_ptr());

            if !SDL_Init(SDL_INIT_VIDEO) {
                dbg_sdl_error("SDL_Init failed");
                return Err(());
            }

            let window = SDL_CreateWindow(title.as_ptr(), 640, 480, SDL_WindowFlags::RESIZABLE);
            if window.is_null() {
                dbg_sdl_error("SDL_CreateWindow failed");
                return Err(());
            }

            let renderer = SDL_CreateRenderer(window, null());
            if renderer.is_null() {
                dbg_sdl_error("SDL_CreateRenderer failed");
                return Err(());
            }

            use sdl3_sys::everything::{SDL_SetRenderDrawColor, SDL_RenderClear, SDL_RenderPresent};

            // render some color
            SDL_SetRenderDrawColor(renderer, 128, 128, 0, 255); // yellow
            SDL_RenderClear(renderer);
            SDL_RenderPresent(renderer);

            return Ok(MyAppState {
                window,
                renderer
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

sdl3_main!(MyAppState);

pub fn dbg_sdl_error(msg: &str) {
    println!("{msg}");
    let error = unsafe { CStr::from_ptr(SDL_GetError()) };
    let error = error.to_string_lossy();
    println!("{error}");
}

