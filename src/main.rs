use std::{fs, path::PathBuf, time::Duration};

use chrono::Local;
use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Sense, Stroke};
use serde::{Deserialize, Serialize};

const BACKGROUND: Color32 = Color32::from_rgb(24, 24, 24);
const SURFACE: Color32 = Color32::from_rgb(32, 32, 32);
const EDGE: Color32 = Color32::from_rgb(62, 62, 62);
const INK: Color32 = Color32::from_rgb(235, 231, 222);
const MUTED: Color32 = Color32::from_rgb(145, 143, 137);
const ACCENT: Color32 = Color32::from_rgb(222, 130, 91);
const GREEN: Color32 = Color32::from_rgb(151, 185, 133);

#[derive(Debug, Deserialize, Serialize)]
struct TodoItem {
    text: String,
    done: bool,
}

struct PixelWid {
    tasks: Vec<TodoItem>,
    new_task: String,
    save_path: Option<PathBuf>,
    status: String,
    status_is_error: bool,
}

impl PixelWid {
    fn new() -> Self {
        let save_path =
            dirs::data_local_dir().map(|directory| directory.join("PixelWid").join("tasks.json"));

        let (tasks, status, status_is_error) = match &save_path {
            Some(path) => match fs::read_to_string(path) {
                Ok(contents) => match serde_json::from_str(&contents) {
                    Ok(tasks) => (tasks, "LOCAL SAVE // READY".to_owned(), false),
                    Err(error) => (
                        Vec::new(),
                        format!("Could not read saved tasks: {error}"),
                        true,
                    ),
                },
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    (Vec::new(), "LOCAL SAVE // NEW BOARD".to_owned(), false)
                }
                Err(error) => (
                    Vec::new(),
                    format!("Could not load saved tasks: {error}"),
                    true,
                ),
            },
            None => (
                Vec::new(),
                "Could not locate a local data directory".to_owned(),
                true,
            ),
        };

        Self {
            tasks,
            new_task: String::new(),
            save_path,
            status,
            status_is_error,
        }
    }

    fn save_tasks(&mut self) {
        let Some(path) = &self.save_path else {
            return;
        };

        let result = (|| {
            if let Some(directory) = path.parent() {
                fs::create_dir_all(directory)?;
            }
            let contents = serde_json::to_string_pretty(&self.tasks)?;
            fs::write(path, contents)
        })();

        match result {
            Ok(()) => {
                self.status = "LOCAL SAVE // OK".to_owned();
                self.status_is_error = false;
            }
            Err(error) => {
                self.status = format!("Could not save tasks: {error}");
                self.status_is_error = true;
            }
        }
    }

    fn add_task(&mut self) {
        let text = self.new_task.trim();
        if text.is_empty() {
            return;
        }

        self.tasks.push(TodoItem {
            text: text.to_owned(),
            done: false,
        });
        self.new_task.clear();
        self.save_tasks();
    }
}

impl eframe::App for PixelWid {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        context.request_repaint_after(Duration::from_secs(1));
        context.set_visuals(egui::Visuals::dark());
        context.style_mut(|style| {
            style.override_font_id = Some(FontId::monospace(13.0));
            style.visuals.override_text_color = Some(INK);
            style.visuals.window_fill = BACKGROUND;
            style.visuals.panel_fill = BACKGROUND;
            style.visuals.widgets.noninteractive.bg_fill = SURFACE;
            style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, INK);
            style.visuals.widgets.inactive.bg_fill = SURFACE;
            style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, INK);
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(48, 43, 40);
            style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, ACCENT);
            style.visuals.widgets.active.bg_fill = ACCENT;
            style.visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, BACKGROUND);
        });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(BACKGROUND)
                    .stroke(Stroke::new(1.0_f32, EDGE)),
            )
            .show(context, |ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let title =
                        ui.label(RichText::new("[ PIXELWID // DESK ]").color(ACCENT).strong());
                    let drag_area =
                        ui.interact(title.rect, ui.id().with("window_drag"), Sense::drag());
                    if drag_area.dragged() {
                        context.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .button(RichText::new("X").color(MUTED))
                            .on_hover_text("Close widget")
                            .clicked()
                        {
                            context.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(12.0);

                ui.label(RichText::new("SYSTEM TIME").color(MUTED).small());
                ui.add_space(2.0);
                ui.label(
                    RichText::new(Local::now().format("%H:%M:%S").to_string())
                        .font(FontId::monospace(38.0))
                        .color(INK)
                        .strong(),
                );
                ui.label(
                    RichText::new(
                        Local::now()
                            .format("%A  //  %d %B %Y")
                            .to_string()
                            .to_uppercase(),
                    )
                    .color(ACCENT)
                    .small(),
                );

                ui.add_space(18.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ ]").color(ACCENT).strong());
                    ui.label(RichText::new("TASK QUEUE").strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{:02}", self.tasks.len()))
                                .color(MUTED)
                                .small(),
                        );
                    });
                });
                ui.add_space(8.0);

                let mut changed = false;
                let mut remove_index = None;
                egui::ScrollArea::vertical()
                    .max_height(205.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.tasks.is_empty() {
                            ui.add_space(8.0);
                            ui.label(RichText::new("> no tasks yet").color(MUTED).italics());
                            ui.add_space(8.0);
                        }

                        for (index, task) in self.tasks.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                let marker = if task.done { "[x]" } else { "[ ]" };
                                if ui
                                    .button(RichText::new(marker).color(if task.done {
                                        GREEN
                                    } else {
                                        ACCENT
                                    }))
                                    .clicked()
                                {
                                    task.done = !task.done;
                                    changed = true;
                                }

                                let task_text = if task.done {
                                    RichText::new(&task.text).color(MUTED).strikethrough()
                                } else {
                                    RichText::new(&task.text).color(INK)
                                };
                                ui.label(task_text);

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui
                                        .small_button(RichText::new("-").color(MUTED))
                                        .on_hover_text("Remove task")
                                        .clicked()
                                    {
                                        remove_index = Some(index);
                                    }
                                });
                            });
                        }
                    });

                if let Some(index) = remove_index {
                    self.tasks.remove(index);
                    changed = true;
                }
                if changed {
                    self.save_tasks();
                }

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.new_task)
                            .hint_text("Type a task...")
                            .desired_width(ui.available_width() - 52.0),
                    );
                    let submit = ui
                        .add_enabled(
                            !self.new_task.trim().is_empty(),
                            egui::Button::new(RichText::new("+ ADD").color(ACCENT).strong()),
                        )
                        .clicked();
                    let submit_from_enter =
                        input.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));

                    if submit || submit_from_enter {
                        self.add_task();
                    }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                let status_color = if self.status_is_error { ACCENT } else { GREEN };
                ui.label(RichText::new(&self.status).color(status_color).small());
                ui.add_space(4.0);
                let completed = self.tasks.iter().filter(|task| task.done).count();
                ui.label(
                    RichText::new(format!(
                        "DONE {completed:02}  //  OPEN {:02}",
                        self.tasks.len() - completed
                    ))
                    .color(MUTED)
                    .small(),
                );
                ui.add_space(8.0);
            });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    if std::env::var_os("WAYLAND_DISPLAY").is_none()
        && std::env::var_os("WAYLAND_SOCKET").is_none()
        && std::env::var_os("DISPLAY").is_none()
    {
        return Err(
            "PixelWid needs a graphical desktop session. Run it on Windows 11, or in Linux with WAYLAND_DISPLAY or DISPLAY set."
                .into(),
        );
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("PixelWid")
            .with_inner_size([360.0, 530.0])
            .with_min_inner_size([320.0, 440.0])
            .with_position([48.0, 72.0])
            .with_decorations(false)
            .with_always_on_top()
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "PixelWid",
        options,
        Box::new(|_creation_context| Ok(Box::new(PixelWid::new()))),
    )?;
    Ok(())
}
