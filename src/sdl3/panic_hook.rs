//! Contains logic to log panic using SDL

/// Setup panic hook so panic is logged through SDL_LogCritical
pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let thread = thread.name().unwrap_or("<unnamed>");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        match info.location() {
            Some(location) => {
                crate::log_critical!(
                    "thread '{}' panicked at '{}': {}:{}",
                    thread,
                    msg,
                    location.file(),
                    location.line(),
                );
            },
            None => {
                println!(
                    "thread '{}' panicked at '{}':",
                    thread,
                    msg,
                );
            },
        }

        std::process::abort();
    }));
}
