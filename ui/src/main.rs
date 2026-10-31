use eframe::egui;
use pyo3::prelude::*;
use pyo3::types::PyModule;

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
    python_result: String,
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

            if ui.button("TEST PYTHON").clicked() {
                self.python_result = Python::with_gil(|py| {
                    let result = (|| -> PyResult<String> {
                        let sys = PyModule::import(py, "sys")?;

                        let cwd = std::env::current_dir().map_err(|e| {
                            pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
                        })?;

                        let cwd = cwd.to_string_lossy().to_string();

                        sys.getattr("path")?.call_method1("insert", (0, cwd))?;

                        let module = PyModule::import(py, "example")?;

                        // Call test_function()
                        let result = module.getattr("test_function")?.call0()?;

                        // Convert returned Python string to Rust String
                        result.extract::<String>()
                    })();

                    match result {
                        Ok(value) => value,
                        Err(err) => format!("Python Error: {}", err),
                    }
                });
            }

            ui.separator();

            ui.label(format!("Counter 1: {}", self.glyph_size));
            ui.label(format!("Counter 2: {}", self.svg_width));
            ui.label(format!("Counter 3: {}", self.svg_height));
            ui.label(format!("Path: {}!", self.path));

            ui.separator();

            ui.heading("Python Output:");
            ui.label(&self.python_result);
        });
    }
}
