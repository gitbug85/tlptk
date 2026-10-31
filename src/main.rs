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
    content: String,
    glyph_size: i32,
    svg_width: i32,
    svg_height: i32,
    python_result: String,
    svg_bytes: Vec<u8>,
    svg_version: u64,
}

impl MyApp {
    fn update_python_result(&mut self) {
        self.python_result = Python::with_gil(|py| {
            let result = (|| -> PyResult<String> {
                let sys = PyModule::import(py, "sys")?;

                let cwd = std::env::current_dir()
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

                sys.getattr("path")?
                    .call_method1("insert", (0, cwd.to_string_lossy().to_string()))?;

                let py_args = vec![
                    "writer.py".to_string(),
                    "-w".to_string(),
                    self.glyph_size.to_string(),
                    "-o".to_string(),
                    self.content.clone(),
                    "-p".to_string(),
                    self.svg_width.to_string(),
                    "-s".to_string(),
                    self.svg_height.to_string(),
                ];

                let sys_argv = sys.getattr("argv")?;
                sys_argv.call_method0("clear")?;

                for arg in &py_args {
                    sys_argv.call_method1("append", (arg,))?;
                }

                let importlib = PyModule::import(py, "importlib")?;
                let module = PyModule::import(py, "writer")?;
                let module = importlib.call_method1("reload", (module,))?;

                let result = module.getattr("main")?.call0()?;

                match result.extract::<String>() {
                    Ok(value) => Ok(value),
                    Err(_) => Ok("Python script executed successfully".to_string()),
                }
            })();

            match result {
                Ok(value) => value,
                Err(err) => format!("Error: {}", err),
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Generate SVG");

            let mut should_update = false;

            ui.horizontal(|ui| {
                ui.label("Content:");

                egui::ScrollArea::vertical()
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        should_update |= ui
                            .add(
                                egui::TextEdit::multiline(&mut self.content)
                                    .desired_width(500.0)
                                    .desired_rows(10),
                            )
                            .changed();
                    });
            });

            should_update |= ui
                .add(egui::Slider::new(&mut self.glyph_size, 1..=200).text("Glyph Size"))
                .changed();

            should_update |= ui
                .add(egui::Slider::new(&mut self.svg_width, 1..=2000).text("SVG Width"))
                .changed();

            should_update |= ui
                .add(egui::Slider::new(&mut self.svg_height, 1..=2000).text("SVG Height"))
                .changed();

            // Regenerate SVG whenever an input changes.
            if should_update {
                self.update_python_result();

                self.svg_bytes = self.python_result.as_bytes().to_vec();
                self.svg_version += 1;
            }

            ui.separator();

            ui.heading("Output:");

            if self.python_result.trim_start().starts_with("<svg") {
                let uri = format!("bytes://python_output_{}.svg", self.svg_version);

                ui.add(
                    egui::Image::from_bytes(uri, self.svg_bytes.clone())
                        .fit_to_exact_size(egui::vec2(500.0, 500.0)),
                );
            } else {
                ui.label(&self.python_result);
            }
        });
    }
}
