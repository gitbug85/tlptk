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
            let mut should_update = false;
            ui.heading("lon SVG li te ./test/output.svg to");

            ui.horizontal(|ui| {
                ui.label("suli pi sitelen wan:");
                should_update |= ui
                    .add(egui::Slider::new(&mut self.glyph_size, 1..=200).show_value(true))
                    .changed();

                ui.label("suli tawa poka:");
                should_update |= ui
                    .add(egui::Slider::new(&mut self.svg_width, 1..=2000).show_value(true))
                    .changed();

                ui.label("suli:");
                should_update |= ui
                    .add(egui::Slider::new(&mut self.svg_height, 1..=2000).show_value(true))
                    .changed();
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

                        if should_update {
                            self.update_python_result();

                            if self.python_result.trim_start().starts_with("<svg") {
                                self.svg_bytes = self.python_result.as_bytes().to_vec();
                                self.svg_version += 1;
                            }
                        }
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::vec2(column_width, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.heading("pana weka:");

                        let uri = format!("bytes://python_output_{}.svg", self.svg_version);

                        ui.add(
                            egui::Image::from_bytes(uri, self.svg_bytes.clone())
                                .fit_to_exact_size(egui::vec2(column_width, column_width)),
                        );
                    },
                );
            });
        });
    }
}
