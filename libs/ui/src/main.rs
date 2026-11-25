use eframe::egui;
use image::error::ParameterErrorKind::DimensionMismatch;
use pdf::save_pdf;
use rfd::FileDialog;
use std::fmt;
use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::ExitStatus;
use std::process::Output;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "tlptk",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    content: String,
    pdf_path: PathBuf,
    writing_sitelen_pona: bool,
    selected: Dimensions,
    path: PathBuf,
    render_output: Output,
    render_texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            content: String::default(),
            pdf_path: PathBuf::default(),
            writing_sitelen_pona: false,
            selected: Dimensions::default(),
            path: PathBuf::default(),
            render_output: Output {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
            render_texture: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Dimensions {
    lili,
    #[default]
    meso,
    suli,
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dimensions::lili => write!(f, "lili"),
            Dimensions::meso => write!(f, "meso"),
            Dimensions::suli => write!(f, "suli"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut should_update = false;

            ui.horizontal(|ui| {
                //                 let label = if !self.writing_sitelen_pona {
                //                     "sitelen pona"
                //                 } else {
                //                     "sitelen lasina"
                //                 };
                //
                //                 let color = if self.writing_sitelen_pona {
                //                     egui::Color32::default()
                //                 } else {
                //                     egui::Color32::BLUE
                //                 };
                //
                //                 if ui
                //                     .selectable_label(
                //                         self.writing_sitelen_pona,
                //                         egui::RichText::new(label).background_color(color),
                //                     )
                //                     .clicked()
                //                 {
                //                     self.writing_sitelen_pona = !self.writing_sitelen_pona;
                //                 }

                if ui.button("o awen e pali").clicked() {
                    fs::write(self.path.clone(), self.content.clone())
                        .expect("Could not write content.");
                    should_update = true;

                    let path = self.path.clone();
                    let size = self.selected;
                    tokio::spawn(async move {
                        let width;
                        let height;
                        if size == Dimensions::lili {
                            width = 107.95;
                            height = 174.498;
                        } else if size == Dimensions::meso {
                            width = 152.4;
                            height = 228.6;
                        } else {
                            width = 215.9;
                            height = 279.4;
                        }
                        save_pdf(path, width * 6.0, height * 6.0).await;
                    });
                }

                ui.heading("suli pi PDF:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, Dimensions::lili, "lili");
                        ui.selectable_value(&mut self.selected, Dimensions::meso, "meso");
                        ui.selectable_value(&mut self.selected, Dimensions::suli, "suli");
                    });

                if ui.button("nasin lon pi ilo sona: ").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path = path.clone();
                        self.pdf_path = path;
                        self.pdf_path.set_extension("pdf");
                        self.content =
                            fs::read_to_string(self.path.clone()).expect("Could not read file.");
                    }
                }

                ui.label(self.path.display().to_string());
            });

            ui.separator();

            ui.horizontal(|ui| {
                let column_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;

                ui.allocate_ui_with_layout(
                    egui::vec2(column_width, 1000.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.label("sitelen pi pana open:");

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.content)
                                    .desired_width(column_width)
                                    .desired_rows(10)
                                    .min_size(egui::vec2(column_width, 1000.0)),
                            );

                            if response.changed() {
                                should_update = true;
                            }
                        });
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::vec2(column_width, 1000.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        if should_update {
                            let render_path = std::env::current_exe()
                                .expect("Failed to get current executable path")
                                .parent()
                                .expect("Failed to get executable parent directory")
                                .join("render");

                            self.render_output = std::process::Command::new(&render_path)
                                .arg(&self.pdf_path)
                                .arg(&self.selected.to_string())
                                .output()
                                .expect("Could not render output.");

                            match image::load_from_memory(&self.render_output.stdout) {
                                Ok(decoded) => {
                                    let size =
                                        [decoded.width() as usize, decoded.height() as usize];

                                    let rgba = decoded.to_rgba8();

                                    let image = egui::ColorImage::from_rgba_unmultiplied(
                                        size,
                                        rgba.as_raw(),
                                    );

                                    self.render_texture = Some(ui.ctx().load_texture(
                                        "render_preview",
                                        image,
                                        egui::TextureOptions::LINEAR,
                                    ));
                                }
                                Err(err) => {
                                    eprintln!("Failed to decode JPEG: {err}");
                                }
                            }
                        }

                        if let Some(texture) = &self.render_texture {
                            ui.add(egui::Image::new(texture).max_size(ui.available_size()));
                        }
                    },
                );
            });
        });
    }
}
