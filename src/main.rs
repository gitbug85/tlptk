use eframe::egui;
use pyo3::prelude::*;
use pyo3::types::PyModule;

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

            ui.add(egui::Slider::new(&mut self.glyph_size, 1..=200).text("Glyph Size"));

            ui.add(egui::Slider::new(&mut self.svg_width, 1..=2000).text("SVG Width"));

            ui.add(egui::Slider::new(&mut self.svg_height, 1..=2000).text("SVG Height"));

            if ui.button("TEST PYTHON").clicked() {
                self.python_result = Python::with_gil(|py| {
                    let result = (|| -> PyResult<String> {
                        let sys = PyModule::import(py, "sys")?;

                        let cwd = std::env::current_dir().map_err(|e| {
                            pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
                        })?;

                        let cwd = cwd.to_string_lossy().to_string();

                        sys.getattr("path")?.call_method1("insert", (0, cwd))?;

                        // Set command-line arguments
                        let glyph_size = self.glyph_size.to_string();
                        let svg_width = self.svg_width.to_string();
                        let svg_height = self.svg_height.to_string();

                        let py_args = vec![
                            "writer.py".to_string(),
                            "-w".to_string(),
                            glyph_size,
                            "-n".to_string(),
                            self.path.clone(),
                            "-p".to_string(),
                            svg_width,
                            "-s".to_string(),
                            svg_height,
                        ];

                        // Update Python's argv with the latest Rust values.
                        let sys = PyModule::import(py, "sys")?;
                        let sys_argv = sys.getattr("argv")?;

                        sys_argv.call_method0("clear")?;

                        for arg in &py_args {
                            sys_argv.call_method1("append", (arg,))?;
                        }

                        // Import and explicitly reload writer.py.
                        let importlib = PyModule::import(py, "importlib")?;
                        let module = PyModule::import(py, "writer")?;

                        let module = importlib.call_method1("reload", (module,))?;

                        // Execute main() with the updated arguments.
                        let result = module.getattr("main")?.call0()?;

                        match result.extract::<String>() {
                            Ok(value) => Ok(value),
                            Err(_) => Ok("Python script executed successfully".to_string()),
                        }
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

            if self.python_result.trim_start().starts_with("<svg") {
                let svg_bytes = self.python_result.as_bytes().to_vec();

                let uri = "bytes://python_output.svg";

                ui.add(
                    egui::Image::from_bytes(uri, svg_bytes)
                        .fit_to_exact_size(egui::vec2(1000.0, 1000.0)),
                );
            } else {
                ui.label(&self.python_result);
            }
        });
    }
}
