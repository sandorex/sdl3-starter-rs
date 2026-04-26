// pub mod ssl_cert;
pub mod sdl3;
pub mod renderer;
pub mod gui;

use sdl3_sys::everything::{SDL_CreateWindow, SDL_DestroyWindow, SDL_INIT_VIDEO, SDL_Init, SDL_SetAppMetadata, SDL_Window, SDL_WindowFlags};
use sdl3_main::AppResult;
use sdl3_sys::events::SDL_Event;

#[derive(Debug, Default)]
pub struct Task {
    pub finished: bool,
    pub text: String,
}

pub struct TaskList<'a> {
    pub tasks: &'a mut [Task],
}

impl<'a> egui::Widget for TaskList<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // allocate more?
        let (_, response) = ui.allocate_exact_size(
            egui::vec2(100.0, 30.0),
            egui::Sense::click()
        );

        for task in self.tasks {
            ui.horizontal(|ui| {
                ui.checkbox(&mut task.finished, "");
// egui::RichText::new("Strikethrough text").strikethrough()
                let button = ui.add(
                    egui::Label::new(
                        if task.finished {
                            egui::RichText::new(&task.text).strikethrough()
                        } else {
                            egui::RichText::new(&task.text)
                        }
                    )
                    // .selectable(false)
                    // .sense(egui::Sense::CLICK)
                    .wrap()
                );

                // toggle the task
                if button.interact(egui::Sense::CLICK).secondary_clicked() {
                    button.context_menu(|ui| {
                        ui.label("hello");
                        // if ui.button("Close").clicked() {
                        //     ui.close();
                        // }
                        //
                        // if ui.button("Something").clicked() {
                        //     ui.close();
                        // }
                    });
                    // task.finished = !task.finished;
                }
            });
        }

        response
    }
}

pub struct MyAppState {
    pub window: *mut SDL_Window,
    pub renderer: renderer::EguiSdl3Glow,
    pub name: String,
    pub age: i32,
    pub tasks: Vec<Task>,
}

impl Drop for MyAppState {
    fn drop(&mut self) {
        unsafe {
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

            let window = SDL_CreateWindow(title.as_ptr(), 640, 480, SDL_WindowFlags::RESIZABLE | SDL_WindowFlags::OPENGL);
            if window.is_null() {
                sdl_error_log("SDL_CreateWindow failed");
                return Err(());
            }

            let renderer = renderer::EguiSdl3Glow::new(window);

            return Ok(MyAppState {
                window,
                renderer,
                name: "??".to_owned(),
                age: 14,
                tasks: vec![
                    Task {
                        finished: false,
                        text: "Get groceries".to_owned(),
                    },
                    Task {
                        finished: false,
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc ultrices rutrum turpis posuere dignissim. In diam lacus, semper eu magna.".to_owned(),
                    },
                    Task {
                        finished: true,
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc ultrices rutrum turpis posuere dignissim. In diam lacus, semper eu magna.".to_owned(),
                    },
                ]
            })
        }
    }

    fn iterate(&mut self) -> AppResult {
        self.renderer.run(self.window, |ui: _| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("TODO");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(TaskList { tasks: &mut self.tasks });
                    // ui.horizontal(|ui| {
                    //     // let name_label = ui.label("Your name: ");
                    //     ui.checkbox(&mut xx, "");
                    //     ui.label("TODO item 1");
                    //     // ui.text_edit_multiline(&mut self.name)
                    //     //     .labelled_by(name_label.id);
                    // });
                    // ui.horizontal(|ui| {
                    //     // let name_label = ui.label("Your name: ");
                    //     ui.checkbox(&mut xx, "");
                    //     ui.add(egui::Label::new("This item is very very long and contains lorem ipsum dolorem et dolorem et quelete").wrap());
                    //     // ui.label("This item is very very long and contains lorem ipsum dolorem et dolorem et quelete").;
                    //     // ui.text_edit_multiline(&mut self.name)
                    //     //     .labelled_by(name_label.id);
                    // });
                });

                // ui.image(egui::include_image!(
                //     "../test.png"
                // ));
            });
        });

        AppResult::Continue
    }

    fn event(&mut self, event: &SDL_Event) -> AppResult {
        use sdl3_sys::events::SDL_EVENT_QUIT;

        match event.event_type() {
            SDL_EVENT_QUIT => return AppResult::Success,
            _ => {},
        }

        self.renderer.event(event);

        AppResult::Continue
    }
}

pub fn sdl_error_log(msg: &str) {
    let err = sdl3::get_error();
    log_critical!("{}: {}", msg, err);
}

// create the required functions
sdl3_main!(MyAppState);
