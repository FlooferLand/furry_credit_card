#![windows_subsystem = "windows"]
extern crate native_windows_gui as nwg;

mod widgets;
mod macros;

use std::sync::Mutex;
use nwg::*;
use crate::widgets::WidgetsContainer;

#[global_allocator]
static A: std::alloc::System = std::alloc::System;

const WINDOW_WIDTH:  i32 = 553;
const WINDOW_HEIGHT: i32 = 240;

#[derive(Default)]
pub struct App {
    window: Window,
    widgets: WidgetsContainer,
    embed: EmbedResource,
    card_number_input: Mutex<TextInput>,
    expiry_date_input: Mutex<TextInput>,
    security_code_input: Mutex<TextInput>
}

impl App {
    fn submit(&self) {
        if let Ok(number) = self.card_number_input.lock() {
            let number = number.text();
            if let Ok(expiry) = self.expiry_date_input.lock() {
                let expiry = expiry.text();
                if let Ok(csv) = self.security_code_input.lock() {
                    let csv = csv.text();
                    let contents = format!("Numbuh: {number}\nexpiwy date: {expiry}\nsecuwity code: {csv}\n\nSend dis to FloofewLand:3");
                    let _ = std::fs::write("./Cwedit cawd info.txt", contents);
                }
            }
        }

        // Message
        modal_info_message(&self.window, "Fanks :3", "Fanks :3\nIsh mine now!");
        stop_thread_dispatch();
    }
}

mod app_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;
    use nwg::WindowFlags;

    pub struct AppUi {
        inner: Rc<App>,
        default_handler: RefCell<Option<EventHandler>>
    }

    impl NativeUi<AppUi> for App {
        fn build_ui(mut app: App) -> Result<AppUi, NwgError> {
            use nwg::Event as E;
            app.embed = match EmbedResource::load(None) {
                Ok(v) => v,
                Err(err) => {
                    println!("{err}");
                    panic!();
                }
            };

            // Controls
            Window::builder()
                .flags(WindowFlags::WINDOW | WindowFlags::VISIBLE | WindowFlags::MAIN_WINDOW)
                .size((WINDOW_WIDTH, WINDOW_HEIGHT))
                .position(((1920 / 2) - (WINDOW_WIDTH / 2), (1080 / 2) - (WINDOW_HEIGHT / 2)))
                .title("Totally Not Malware")
                .icon(load_icon!(app.embed, "MAIN_ICON"))
                .build(&mut app.window)?;

            // :3 image
            ImageFrame::builder()
                .size((240, 240))
                .position((0, 0))
                .bitmap(load_bitmap!(app.embed, "CUTE_IMAGE", (180, 180)))
                .parent(&app.window)
                .build(&mut app.widgets.image)?;

            let x_offset = 64;

            // Beg text
            let mut text = Default::default();
            Label::builder()
                .size((235, 80))
                .position((x_offset + 215, 2))
                .text("H-hi there...\nDo th-think I could have your credit card information, p-please?")
                .h_align(HTextAlign::Center)
                .parent(&app.window)
                .build(&mut text)?;
            app.widgets.labels.push(text);

            // Text boxes
            let mut y_offset = 74;
            make_text_input!(app, x_offset, y_offset, "Card number", app.card_number_input, true);
            make_text_input!(app, x_offset, { y_offset += 43; y_offset }, "Expiry date", app.expiry_date_input, false);
            make_text_input!(app, x_offset, { y_offset += 43; y_offset }, "Security code", app.security_code_input, false);

            // Submit button
            Button::builder()
                .size((98, 35))
                .position((x_offset + 280, { y_offset += 45; y_offset }))
                .text("Th-thanks")
                .parent(&app.window)
                .build(&mut app.widgets.thanks_button)?;

            // Wrap-up
            let ui = AppUi {
                inner: Rc::new(app),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick =>
                            if &handle == &ui.widgets.thanks_button {
                                App::submit(&ui);
                            },
                        E::OnTextInput =>
                            if let Ok(input) = ui.card_number_input.lock() {
                                if &handle == &input.handle {
                                    // TODO: FIXME: Can't set the text on the input box because it freezes up
                                    /*let mut clean_text = input.text();
                                    clean_text = clean_text.chars().filter(|c| c.is_ascii_digit()).collect();

                                    let mut segments = [0, 0, 0, 0];
                                    for i in 0..clean_text.len() {
                                        if (i > 0 && i % 4 == 0) {
                                            segments[(i - 1) / 4] = clean_text[i - 4..i].parse::<i32>().unwrap();
                                        }
                                    }

                                    input.set_text(&segments.map(|e| e.to_string()).join(" "));*/
                                }
                            },

                        E::OnWindowClose =>
                            stop_thread_dispatch(),
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(full_bind_event_handler(&ui.window.handle, handle_events));
            return Ok(ui);
        }
    }

    impl Drop for AppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for AppUi {
        type Target = App;
        fn deref(&self) -> &App {
            &self.inner
        }
    }
}

fn main() {
    init().expect("Failed to init Native Windows GUI");
    let _ = Font::set_global_family("Segoe UI");
    let _ui = App::build_ui(Default::default()).expect("Failed to build UI");
    dispatch_thread_events();
}
