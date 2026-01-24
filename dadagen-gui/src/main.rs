//! Cross-platform GUI application for dadagen
//! 
//! Built with egui for pure Rust native UI across all platforms.
//! 
//! Features:
//! - File upload with drag-and-drop support (CSV, JSON, Excel)
//! - Data preview and analysis
//! - Automatic DSL generation from sample data
//! - Real-time DSL editing with syntax highlighting
//! - Data generation preview
//! - Export functionality for DSL and generated data

use eframe::egui;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    // Set up logging
    tracing_subscriber::fmt::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Dadagen - Test Data Generator")
            .with_drag_and_drop(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Dadagen",
        options,
        Box::new(|cc| {
            // Set up custom fonts if needed
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(DadagenApp::new(cc)))
        }),
    )
}

#[derive(Default, PartialEq, Eq)]
enum AppView {
    #[default]
    FileInput,
    DslEditor,
    DataPreview,
}

struct DadagenApp {
    // File handling
    file_path: Option<PathBuf>,
    file_content: String,
    file_type: FileType,
    
    // DSL generation
    dsl_output: String,
    dsl_editable: bool,
    
    // UI state
    current_view: AppView,
    show_settings: bool,
    show_about: bool,
    
    // Settings
    theme_dark: bool,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum FileType {
    #[default]
    Unknown,
    Csv,
    Json,
    Excel,
}

impl DadagenApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_path: None,
            file_content: String::new(),
            file_type: FileType::Unknown,
            dsl_output: String::new(),
            dsl_editable: true,
            current_view: AppView::FileInput,
            show_settings: false,
            show_about: false,
            theme_dark: true,
        }
    }
}

impl eframe::App for DadagenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.theme_dark {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📁 Open File...").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save DSL As...").clicked() {
                        self.save_dsl();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("⚙️ Settings").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("📂 File Input").clicked() {
                        self.current_view = AppView::FileInput;
                        ui.close_menu();
                    }
                    if ui.button("🔧 DSL Editor").clicked() {
                        self.current_view = AppView::DslEditor;
                        ui.close_menu();
                    }
                    if ui.button("📊 Data Preview").clicked() {
                        self.current_view = AppView::DataPreview;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("📖 Documentation").clicked() {
                        let _ = open::that("https://github.com/inosion/dadagen");
                        ui.close_menu();
                    }
                    if ui.button("ℹ️ About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
                
                // Spacer
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Dadagen v0.1.0");
                });
            });
        });
        
        // Tab navigation
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_view, AppView::FileInput, "📂 File Input");
                ui.selectable_value(&mut self.current_view, AppView::DslEditor, "🔧 DSL Editor");
                ui.selectable_value(&mut self.current_view, AppView::DataPreview, "📊 Data Preview");
            });
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::FileInput => self.show_file_input(ctx, ui),
                AppView::DslEditor => self.show_dsl_editor(ctx, ui),
                AppView::DataPreview => self.show_data_preview(ctx, ui),
            }
        });
        
        // Settings dialog
        if self.show_settings {
            self.show_settings_dialog(ctx);
        }
        
        // About dialog  
        if self.show_about {
            self.show_about_dialog(ctx);
        }
        
        // Handle drag and drop
        self.handle_drag_drop(ctx);
    }
}

impl DadagenApp {
    // View implementations
    fn show_file_input(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("📂 File Input & Analysis");
        ui.add_space(10.0);
        
        // File selection area
        ui.group(|ui| {
            ui.set_min_height(150.0);
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                if let Some(path) = &self.file_path {
                    ui.label(egui::RichText::new(format!("📄 {}", path.file_name().unwrap().to_string_lossy()))
                        .size(16.0));
                    ui.label(egui::RichText::new(path.parent().unwrap().display().to_string())
                        .small()
                        .color(egui::Color32::GRAY));
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("🔄 Change File").clicked() {
                            self.open_file();
                        }
                        if ui.button("🔧 Generate DSL").clicked() {
                            self.generate_dsl();
                            self.current_view = AppView::DslEditor;
                        }
                    });
                } else {
                    ui.label(egui::RichText::new("📁 Drag & Drop File Here")
                        .size(20.0)
                        .color(egui::Color32::GRAY));
                    ui.label("or");
                    if ui.button(egui::RichText::new("📂 Browse...").size(16.0)).clicked() {
                        self.open_file();
                    }
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Supported formats: CSV, JSON, Excel")
                        .small()
                        .color(egui::Color32::GRAY));
                }
                
                ui.add_space(20.0);
            });
        });
        
        ui.add_space(15.0);
        
        // File preview
        if !self.file_content.is_empty() {
            ui.heading("Preview");
            ui.separator();
            
            egui::ScrollArea::vertical()
                .max_height(500.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.file_content.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(false)
                    );
                });
        }
    }
    
    fn show_dsl_editor(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("🔧 DSL Editor");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.checkbox(&mut self.dsl_editable, "✏️ Editable");
                if ui.button("📋 Copy").clicked() {
                    ui.output_mut(|o| o.copied_text = self.dsl_output.clone());
                }
                if ui.button("💾 Save").clicked() {
                    self.save_dsl();
                }
            });
        });
        
        ui.separator();
        ui.add_space(5.0);
        
        if self.dsl_output.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.label(egui::RichText::new("No DSL generated yet")
                    .size(18.0)
                    .color(egui::Color32::GRAY));
                ui.add_space(10.0);
                ui.label("Upload a file and click 'Generate DSL' to get started");
                if ui.button("Go to File Input").clicked() {
                    self.current_view = AppView::FileInput;
                }
            });
        } else {
            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.dsl_output)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(40)
                            .interactive(self.dsl_editable)
                    );
                });
        }
    }
    
    fn show_data_preview(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("📊 Generated Data Preview");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Regenerate").clicked() {
                    // TODO: Regenerate data
                }
                if ui.button("📥 Export").clicked() {
                    // TODO: Export data
                }
            });
        });
        
        ui.separator();
        ui.add_space(5.0);
        
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.label(egui::RichText::new("Data generation preview coming soon")
                .size(18.0)
                .color(egui::Color32::GRAY));
            ui.add_space(10.0);
            ui.label("This will show a table preview of generated test data");
        });
    }
    
    fn show_settings_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Settings")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Appearance");
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    ui.radio_value(&mut self.theme_dark, true, "🌙 Dark");
                    ui.radio_value(&mut self.theme_dark, false, "☀️ Light");
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                if ui.button("Close").clicked() {
                    self.show_settings = false;
                }
            });
    }
    
    fn show_about_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("ℹ️ About Dadagen")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Dadagen");
                    ui.label("Test Data Generator");
                    ui.label("Version 0.1.0");
                    ui.add_space(10.0);
                    ui.label("Built with ❤️ using egui");
                    ui.add_space(10.0);
                    ui.hyperlink_to("GitHub", "https://github.com/inosion/dadagen");
                });
                
                ui.add_space(10.0);
                
                if ui.button("Close").clicked() {
                    self.show_about = false;
                }
            });
    }
    
    // File operations
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
        // Determine file type from extension
        self.file_type = match path.extension().and_then(|e| e.to_str()) {
            Some("csv") => FileType::Csv,
            Some("json") => FileType::Json,
            Some("xlsx") | Some("xls") => FileType::Excel,
            _ => FileType::Unknown,
        };
        
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.file_path = Some(path);
                // Limit preview to first 10,000 chars
                self.file_content = if content.len() > 10000 {
                    format!("{}...\n\n(Preview truncated - showing first 10,000 characters)", &content[..10000])
                } else {
                    content
                };
                
                tracing::info!("Loaded file: {:?}", self.file_path);
            }
            Err(e) => {
                self.file_content = format!("❌ Error reading file: {}", e);
                self.file_path = None;
                tracing::error!("Failed to load file: {}", e);
            }
        }
    }
    
    fn generate_dsl(&mut self) {
        // TODO: Integrate with dadagen-core to parse file and generate DSL
        // For now, this is a placeholder implementation
        
        let file_name = self.file_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        self.dsl_output = format!(
            "# Dadagen DSL Configuration\n\
             # Generated from: {}\n\
             # File type: {:?}\n\
             # Date: {}\n\n\
             # TODO: Implement actual DSL generation by:\n\
             # 1. Parsing the file structure (CSV/JSON/Excel)\n\
             # 2. Inferring data types from sample data\n\
             # 3. Generating appropriate field definitions\n\
             # 4. Creating realistic generators based on patterns\n\n\
             # Example output:\n\
             schema {} {{\n\
             \tid: counter(start: 1, step: 1)\n\
             \tname: firstname + \" \" + lastname\n\
             \temail: lowercase(firstname) + \".\" + lowercase(lastname) + \"@example.com\"\n\
             \tage: integer(18, 99)\n\
             \tcreated_at: datetime(\"2020-01-01\", \"2024-12-31\")\n\
             \tis_active: boolean(0.8)\n\
             }}\n",
            file_name,
            self.file_type,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            file_name.replace(".", "_")
        );
        
        tracing::info!("Generated DSL placeholder");
    }
    
    fn save_dsl(&self) {
        if self.dsl_output.is_empty() {
            return;
        }
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Dadagen DSL", &["dadagen"])
            .set_file_name("generated.dadagen")
            .save_file()
        {
            match std::fs::write(&path, &self.dsl_output) {
                Ok(_) => {
                    tracing::info!("DSL saved to: {}", path.display());
                }
                Err(e) => {
                    tracing::error!("Error saving file: {}", e);
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
                        // Auto-switch to file input view to show the loaded file
                        self.current_view = AppView::FileInput;
                    }
                }
            }
        });
    }
}
