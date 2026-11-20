use eframe::egui;
use rfd::FileDialog;

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
    path: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut should_update = false;
            ui.heading("lon SVG li te ./test/output.svg to");

            ui.horizontal(|ui| {
                ui.label("nasin lon pi ilo sona:");
                ui.add_sized(
                    [200.0, ui.spacing().interact_size.y],
                    egui::TextEdit::singleline(&mut self.path),
                );
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
