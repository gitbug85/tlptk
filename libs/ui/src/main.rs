use eframe::egui;
use pdf::save_pdf;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
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

#[derive(Default)]
struct MyApp {
    content: String,
    path: PathBuf,
    writing_sitelen_pona: bool,
    selected: Dimensions,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Dimensions {
    lili,
    #[default]
    meso,
    suli,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut should_update = false;

            ui.horizontal(|ui| {
                let label = if !self.writing_sitelen_pona {
                    "sitelen pona"
                } else {
                    "sitelen lasina"
                };

                let color = if self.writing_sitelen_pona {
                    egui::Color32::default()
                } else {
                    egui::Color32::BLUE
                };

                if ui
                    .selectable_label(
                        self.writing_sitelen_pona,
                        egui::RichText::new(label).background_color(color),
                    )
                    .clicked()
                {
                    self.writing_sitelen_pona = !self.writing_sitelen_pona;
                }

                if ui.button("o awen e pali").clicked() {
                    save_pdf(self.path.clone())
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
                        self.path = path;
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
                            should_update |= ui
                                .add(
                                    egui::TextEdit::multiline(&mut self.content)
                                        .desired_width(column_width)
                                        .desired_rows(10)
                                        .min_size(egui::vec2(column_width, 1000.0)),
                                )
                                .changed();
                        });
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::vec2(column_width, 1000.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.label("Preview:");

                        let render_path = std::env::current_exe()
                            .expect("Failed to get current executable path")
                            .parent()
                            .expect("Failed to get executable parent directory")
                            .join("render");

                        println!("Render path: {}", render_path.display());
                        println!("Input path: {}", self.path.display());

                        let output = std::process::Command::new(&render_path)
                            .arg(&self.path)
                            .output();

                        match output {
                            Ok(output) if output.status.success() => {
                                match image::load_from_memory(&output.stdout) {
                                    Ok(decoded) => {
                                        let size =
                                            [decoded.width() as usize, decoded.height() as usize];

                                        let rgba = decoded.to_rgba8();
                                        let pixels = rgba.as_flat_samples();

                                        let image = egui::ColorImage::from_rgba_unmultiplied(
                                            size,
                                            pixels.as_slice(),
                                        );

                                        let texture = ui.ctx().load_texture(
                                            "render_preview",
                                            image,
                                            egui::TextureOptions::LINEAR,
                                        );

                                        ui.add(
                                            egui::Image::new(&texture)
                                                .max_size(ui.available_size()),
                                        );
                                    }
                                    Err(err) => {
                                        ui.label(format!("Failed to decode JPEG: {err}"));
                                    }
                                }
                            }
                            Ok(output) => {
                                ui.label(format!(
                                    "Render failed: {}",
                                    String::from_utf8_lossy(&output.stderr)
                                ));
                            }
                            Err(err) => {
                                ui.label(format!("Failed to execute render: {:?}", self.path));
                            }
                        }
                    },
                );
            });
        });
    }
}
