use eframe::egui;
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
    path: Option<PathBuf>,
    writing_sitelen_pona: bool,
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

                if ui.button("nasin lon pi ilo sona: ").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path = Some(path);
                    }
                }

                if let Some(path) = &self.path {
                    ui.label(path.display().to_string());
                }
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
            });
        });
    }
}
