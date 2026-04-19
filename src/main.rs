mod sdl_helper;

use std::{ffi::{CStr, CString, OsStr}, path::{Path, PathBuf}, ptr::null};
use git2::{CertificateCheckStatus, build::RepoBuilder};
use sdl3_sys::{everything::{SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_GetError, SDL_INIT_VIDEO, SDL_Init, SDL_Renderer, SDL_SetAppMetadata, SDL_Window, SDL_WindowFlags}, filesystem::{SDL_Folder, SDL_GetUserFolder}, log::SDL_LOG_CATEGORY_SYSTEM};
use sdl3_main::AppResult;
use sdl3_sys::events::SDL_Event;

struct MyAppState {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    cloned: bool,
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
                renderer,
                cloned: false,
            })
        }
    }

    fn iterate(&mut self) -> AppResult {
        // sdl_sys:: SDL_RequestAndroidPermission
        // SDL_RequestAndroidPermission
        // sdl3_sys::everything::SDL_RequestAndroidPermission
        // if cfg!(ANDROID) {
        //     // sdl3_sys::everything::SDL_Request
        // }
        // let dir_path = SDL_GetUserFolder(SDL_Folder::DOWNLOADS);
        // let dir_path = SDL_AndroidGetExternalStoragePath;

        if !self.cloned {
            self.cloned = true;

            let dir_path = unsafe { sdl3_sys::filesystem::SDL_GetPrefPath(c"org.example".as_ptr(), c"sdl3-starter".as_ptr()) };

            if dir_path.is_null() {
                dbg_sdl_error("SDL_GetUserFolder failed");
                return AppResult::Failure;
            }

            // log the path
            unsafe { sdl3_sys::log::SDL_LogError(SDL_LOG_CATEGORY_SYSTEM.into(), dir_path) };

            let dir_path_cstr = unsafe { CStr::from_ptr(dir_path) };
            let dir_path = Path::new(dir_path_cstr.to_str().unwrap());


            let url = "https://github.com/sandorex/arcam";
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.certificate_check(|_cert, _host| {
                // TODO this is as the certificates are broken on android build for some reason?
                // Return true to accept the certificate despite errors
                Ok(CertificateCheckStatus::CertificateOk)
            });

            let mut fetch_options = git2::FetchOptions::new();
            fetch_options.remote_callbacks(callbacks);

            // let repo = match Repository::clone(url, dir_path_cstr.to_string_lossy().to_string()) {
            let repo = match RepoBuilder::new().fetch_options(fetch_options).clone(url, dir_path) {
                Ok(repo) => repo,
                Err(e) => {
                    dbg_sdl_error(&format!("Repository::clone failed: {e}"));
                    return AppResult::Failure;
                },
            };
        }

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
    // let error = unsafe { CStr::from_ptr(SDL_GetError()) };
    // let error = error.to_string_lossy();

    let msg = CString::new(msg).expect("could not convert msg to cstring");
    unsafe { sdl3_sys::log::SDL_LogError(SDL_LOG_CATEGORY_SYSTEM.into(), msg.as_ptr()) };
    unsafe { sdl3_sys::log::SDL_LogError(SDL_LOG_CATEGORY_SYSTEM.into(), SDL_GetError()) };
    // unsafe { sdl3_sys::log::SDL_Log(SDL_GetError()) };
}

