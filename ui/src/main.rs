use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "tlptk",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    path: String,
    glyph_size: i32,
    svg_width: i32,
    svg_height: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Generate SVG");

            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.text_edit_singleline(&mut self.path);
            });

            if ui.button("Increment glyph_size").clicked() {
                self.glyph_size += 1;
            }

            if ui.button("Increment svg_width").clicked() {
                self.svg_width += 1;
            }

            if ui.button("Increment svg_height").clicked() {
                self.svg_height += 1;
            }

            ui.label(format!("Counter 1: {}", self.glyph_size));
            ui.label(format!("Counter 2: {}", self.svg_width));
            ui.label(format!("Counter 3: {}", self.svg_height));
            ui.label(format!("Path, {}!", self.path));
        });
    }
}
