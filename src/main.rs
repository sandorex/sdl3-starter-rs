use std::{ffi::CStr, ptr::null};

use sdl3_sys::everything::{SDL_Init, SDL_INIT_VIDEO, SDL_CreateWindow, SDL_Renderer, SDL_Window, SDL_DestroyWindow, SDL_DestroyRenderer, SDL_WindowFlags, SDL_CreateRenderer, SDL_SetAppMetadata, SDL_GetError};

use sdl3_main::{app_impl, AppResult};
use sdl3_sys::events::SDL_Event;
use std::sync::Mutex;

struct MyAppState {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
}

#[app_impl]
impl MyAppState {
    fn app_init() -> Option<Box<Mutex<MyAppState>>> {
        let title = c"Heeyy";

        unsafe {
            // SDL_SetAppMetadata(c"SDL3 Starter Rust".as_ptr(), c"1.0".as_ptr(), c"com.example.sdl3-starter-rs".as_ptr());

            if !SDL_Init(SDL_INIT_VIDEO) {
                dbg_sdl_error("SDL_Init failed");
                return None;
            }

            let window = SDL_CreateWindow(title.as_ptr(), 640, 480, SDL_WindowFlags::RESIZABLE);
            if window.is_null() {
                dbg_sdl_error("SDL_CreateWindow failed");
                return None;
            }

            let renderer = SDL_CreateRenderer(window, null());
            if renderer.is_null() {
                dbg_sdl_error("SDL_CreateRenderer failed");
                return None;
            }

            use sdl3_sys::everything::{SDL_SetRenderDrawColor, SDL_RenderClear, SDL_RenderPresent};

            // render some color
            SDL_SetRenderDrawColor(renderer, 128, 128, 0, 255); // Black
            SDL_RenderClear(renderer);
            SDL_RenderPresent(renderer); // This forces the display

            return Some(Box::new(Mutex::new(MyAppState {
                window,
                renderer,
            })))
        }
    }

    fn app_iterate(&mut self) -> AppResult {
        AppResult::Continue
    }

    fn app_event(&mut self, event: &SDL_Event) -> AppResult {
        use sdl3_sys::events::{SDL_EVENT_QUIT, SDL_EVENT_KEY_UP, SDL_EVENT_KEY_DOWN, SDL_EVENT_WINDOW_RESIZED};

        match event.event_type() {
            SDL_EVENT_QUIT => return AppResult::Success,
            SDL_EVENT_WINDOW_RESIZED => unsafe { println!("resize {:?}x{:?}", event.window.data1, event.window.data2) },
            _ => {},
        }

        AppResult::Continue
    }
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

unsafe impl Send for MyAppState {}

// #[sdl3_main::main]
// fn main() {
//     println!("Creating window..");
//
//     let title = c"Heeyy";
//
//     unsafe {
//         SDL_SetAppMetadata(c"SDL3 Starter Rust".as_ptr(), c"1.0".as_ptr(), c"com.example.sdl3-starter-rs".as_ptr());
//
//         if !SDL_Init(SDL_INIT_VIDEO) {
//             dbg_sdl_error("SDL_Init failed");
//             return
//         }
//
//         let window = SDL_CreateWindow(title.as_ptr(), 640, 480, SDL_WINDOW_RESIZABLE);
//         if window.is_null() {
//             dbg_sdl_error("SDL_CreateWindow failed");
//             return;
//         }
//
//         // let format_flags =
//         //     SDL_GPU_SHADERFORMAT_SPIRV | SDL_GPU_SHADERFORMAT_DXIL | SDL_GPU_SHADERFORMAT_MSL;
//         // let device = SDL_CreateGPUDevice(format_flags, true, null());
//         // if device.is_null() {
//         //     dbg_sdl_error("SDL_CreateGPUDevice failed");
//         //     return;
//         // }
//         // if !SDL_ClaimWindowForGPUDevice(device, window) {
//         //     dbg_sdl_error("SDL_ClaimWindowForGPUDevice failed");
//         //     return;
//         // }
//     }
// }

pub fn dbg_sdl_error(msg: &str) {
    println!("{msg}");
    let error = unsafe { CStr::from_ptr(SDL_GetError()) };
    let error = error.to_string_lossy();
    println!("{error}");
}
