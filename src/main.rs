#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::Utc;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs, thread};

use clipboard_rs::{Clipboard, ClipboardContext};
use eframe::egui;
use egui::{containers::*, *};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    let mut window_position = Pos2::new(-1.0, -1.0);

    if args.len() > 2 {
        window_position.x = args[1].parse().unwrap_or(-1.0);
        window_position.y = args[2].parse().unwrap_or(-1.0);
    }

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([320.0, 150.0])
            .with_always_on_top()
            .with_resizable(false)
            .with_active(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_position(window_position)
            .with_icon(load_icon()),
        follow_system_theme: true,
        centered: window_position.x < 0f32 || window_position.y < 0f32,
        ..Default::default()
    };
    eframe::run_native(
        "GPT Commander",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    response: Arc<Mutex<String>>,
    clipboard_ctx: ClipboardContext,
    user_content: String,
    is_sent: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            response: Arc::new(Mutex::new("".to_string())),
            clipboard_ctx: ClipboardContext::new().unwrap(),
            user_content: "".to_string(),
            is_sent: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(Duration::from_millis(33));

            let response_clone = Arc::clone(&self.response);
            ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                if self.response.clone().lock().unwrap().clone().len() > 0 {
                    ui.add(
                        TextEdit::multiline(&mut *response_clone.lock().unwrap())
                            .frame(false)
                            .desired_rows(8),
                    );
                } else {
                    let color = if ui.visuals().dark_mode {
                        Color32::from_additive_luminance(196)
                    } else {
                        Color32::from_black_alpha(240)
                    };

                    Frame::canvas(ui.style()).show(ui, |ui| {
                        ui.ctx().request_repaint();
                        let time = ui.input(|i| i.time);
                        let desired_size = ui.available_width() * vec2(1.0, 0.35);
                        let (_id, rect) = ui.allocate_space(desired_size);
                        let to_screen = emath::RectTransform::from_to(
                            Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                            rect,
                        );
                        let mut shapes = vec![];

                        for &mode in &[2, 3, 5] {
                            let mode = mode as f64;
                            let n = 120;
                            let speed = 1.5;

                            let points: Vec<Pos2> = (0..=n)
                                .map(|i| {
                                    let t = i as f64 / (n as f64);
                                    let amp = (time * speed * mode).sin() / mode;
                                    let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                                    to_screen * pos2(t as f32, y as f32)
                                })
                                .collect();

                            let thickness = 10.0 / mode as f32;
                            shapes.push(Shape::line(points, Stroke::new(thickness, color)));
                        }

                        ui.painter().extend(shapes);
                    });
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .add(Button::new("A-OK-dokie").fill(Color32::from_rgb(22, 90, 76)))
                    .clicked()
                {
                    let improve_text = self.response.lock().unwrap().clone();
                    self.clipboard_ctx.set_text(improve_text.clone()).unwrap();

                    if self.user_content.len() > 0 && improve_text.len() > 0 {
                        let current_time = Utc::now().timestamp_millis();

                        // Ensure 'history' directory exists
                        fs::create_dir_all("history").unwrap_or_default();

                        // Save self.user_content
                        let user_file_path =
                            Path::new("history").join(format!("{}_user.txt", current_time));
                        let mut user_file =
                            File::create(&user_file_path).expect("Could not create file");
                        user_file
                            .write_all(self.user_content.as_bytes())
                            .expect("Could not write to file");

                        // Save improve_text
                        let assistant_file_path =
                            Path::new("history").join(format!("{}_assistant.txt", current_time));
                        let mut assistant_file =
                            File::create(&assistant_file_path).expect("Could not create file");
                        assistant_file
                            .write_all(improve_text.as_bytes())
                            .expect("Could not write to file");
                    }

                    std::process::exit(0);
                }
                // if ui.button("Retry").clicked() {
                //     // Mà cớ sao ta giờ đây bước qua đời nhau?
                //     if let Ok(msg) = self.send_request() {
                //         self.response = msg;
                //     }
                // }
                if ui.button("Cancel").clicked() {
                    std::process::exit(0);
                }
            });
        });

        if !self.is_sent {
            self.is_sent = true;
            let response_clone = Arc::clone(&self.response);
            let content = self.get_clipboard_content();

            self.user_content = content.clone();

            thread::spawn(move || {
                let result = send_request(content);
                if let Ok(data) = result {
                    let mut response = response_clone.lock().unwrap();
                    *response = data;
                }
            });
        }
    }
}

impl MyApp {
    fn get_clipboard_content(&self) -> String {
        return self.clipboard_ctx.get_text().unwrap_or("".to_string());
    }
}

fn send_request(content: String) -> anyhow::Result<String> {
    let model = env::var("GPT_COMMANDER_MODEL").unwrap_or("gpt-3.5-turbo".to_string());
    let mut req = ChatCompletionRequest::new(model,
    vec![
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::system,
        content: chat_completion::Content::Text(String::from(include_str!("../autoit/fine-tuning/sysprompt.txt"))),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>ts quick select</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("Quick select implementation in TypeScript")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>rust windows close window title \"%1 macro manager</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("Rust WinAPI close window title: \"%1 macro manager\"")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>Help me convert a tailwind CSS class to MUI sx attribute. My tailwind class is:<br><br>```<br><div class=\"flex\"><br>```</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("Help me convert a tailwind CSS class to MUI sx attribute. My tailwind class is:\n\n```\n<div class=\"flex\">\n```")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>Gà có mấy chân?</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("How many legs does a chicken have?")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>Hoàng sa, trường sa của nước nào?</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("Which country do Hoang Sa and Truong Sa belong to?")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("<document>How old is Bác Hồ?</document>")),
        name: None,
      },
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::assistant,
        content: chat_completion::Content::Text(String::from("How old is Uncle Ho (referring to Ho Chi Minh)?")),
        name: None,
      },
      //
      // chat_completion::ChatCompletionMessage {
      //     role: chat_completion::MessageRole::user,
      //     content: chat_completion::Content::Text(String::from("")),
      //     name: None,
      // },
      // chat_completion::ChatCompletionMessage {
      //     role: chat_completion::MessageRole::assistant,
      //     content: chat_completion::Content::Text(String::from("")),
      //     name: None,
      // },
      //
      chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(format!("<document>{}</document>", content)),
        name: None,
      },
    ],
  );
    req.temperature = Some(0.0);

    let open_ai_key = env::var("OPENAI_API_KEY").unwrap_or("".to_string());

    if open_ai_key.len() == 0 {
        return Ok("You need to set the 'OPENAI_API_KEY' in the system environment.".to_string());
    }

    let client = Client::new(open_ai_key);
    let result = client.chat_completion(req);

    if result.is_err() {
        return Ok("An error occurred while calling the OpenAI API.".to_string());
    }

    Ok(<Option<String> as Clone>::clone(&result?.choices[0].message.content).unwrap())
}

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../autoit/icon.ico");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
