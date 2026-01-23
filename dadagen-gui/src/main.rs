//! Cross-platform GUI application for dadagen
//! 
//! Built with egui for pure Rust native UI across all platforms.

use eframe::egui;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Dadagen - Test Data Generator"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Dadagen",
        options,
        Box::new(|_cc| Ok(Box::new(DadagenApp::default()))),
    )
}

struct DadagenApp {
    file_path: Option<PathBuf>,
    file_content: String,
    dsl_output: String,
}

impl Default for DadagenApp {
    fn default() -> Self {
        Self {
            file_path: None,
            file_content: String::new(),
            dsl_output: String::new(),
        }
    }
}

impl eframe::App for DadagenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📁 Open File...").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        // Left panel - File input and preview
        egui::SidePanel::left("file_panel")
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("📂 File Input");
                ui.add_space(5.0);
                
                if let Some(path) = &self.file_path {
                    ui.label(format!("📄 {}", path.display()));
                } else {
                    ui.colored_label(
                        egui::Color32::GRAY,
                        "No file selected - drag and drop or click to open"
                    );
                }
                
                if ui.button("📁 Select File").clicked() {
                    self.open_file();
                }
                
                ui.separator();
                
                ui.heading("Preview");
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.file_content.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                        );
                    });
            });

        // Central panel - DSL output
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔧 Generated DSL Configuration");
            ui.add_space(5.0);
            
            if ui.button("🔄 Generate DSL from File").clicked() {
                self.generate_dsl();
            }
            
            ui.separator();
            
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.dsl_output)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                    );
                });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("💾 Save DSL").clicked() {
                    self.save_dsl();
                }
                if ui.button("📋 Copy to Clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = self.dsl_output.clone());
                }
            });
        });

        // Handle drag and drop
        self.handle_drag_drop(ctx);
    }
}

impl DadagenApp {
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV Files", &["csv"])
            .add_filter("JSON Files", &["json"])
            .add_filter("Excel Files", &["xlsx", "xls"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            self.load_file(path);
        }
    }
    
    fn load_file(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.file_path = Some(path);
                // Limit preview to first 5000 chars
                self.file_content = if content.len() > 5000 {
                    format!("{}...\n\n(Preview truncated - showing first 5000 characters)", &content[..5000])
                } else {
                    content
                };
            }
            Err(e) => {
                self.file_content = format!("❌ Error reading file: {}", e);
                self.file_path = None;
            }
        }
    }
    
    fn generate_dsl(&mut self) {
        // TODO: Integrate with dadagen-core to parse file and generate DSL
        // This is a placeholder implementation
        self.dsl_output = format!(
            "# Dadagen DSL Configuration\n\
             # Generated from: {}\n\
             # Date: {}\n\n\
             # TODO: Implement actual DSL generation by:\n\
             # 1. Parsing the file structure (CSV/JSON/Excel)\n\
             # 2. Inferring data types from sample data\n\
             # 3. Generating appropriate field definitions\n\
             # 4. Creating realistic generators based on patterns\n\n\
             # Example output:\n\
             schema MyData {{\n\
             \tfield id: integer(1, 10000)\n\
             \tfield name: firstname + \" \" + lastname\n\
             \tfield email: lowercase(firstname) + \".\" + lowercase(lastname) + \"@example.com\"\n\
             \tfield created: date(\"2020-01-01\", \"2024-12-31\")\n\
             }}\n",
            self.file_path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "no file".to_string()),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
    }
    
    fn save_dsl(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Dadagen DSL", &["dadagen"])
            .set_file_name("generated.dadagen")
            .save_file()
        {
            match std::fs::write(&path, &self.dsl_output) {
                Ok(_) => {
                    println!("✅ DSL saved to: {}", path.display());
                }
                Err(e) => {
                    eprintln!("❌ Error saving file: {}", e);
                }
            }
        }
    }
    
    fn handle_drag_drop(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(file) = i.raw.dropped_files.first() {
                    if let Some(path) = &file.path {
                        self.load_file(path.clone());
                    }
                }
            }
        });
    }
}
