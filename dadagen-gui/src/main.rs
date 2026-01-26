
/// Cross-platform GUI application for dadagen
/// 
/// Built with egui for pure Rust native UI across all platforms.
/// 
/// Features:
/// - File upload with drag-and-drop support (CSV, JSON, Excel)
/// - Data preview and analysis
/// - Automatic DSL generation from sample data
/// - Real-time DSL editing with syntax highlighting
/// - Data generation preview
/// - Export functionality for DSL and generated data

use eframe::egui;
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context as AnyhowContext};
use csv::ReaderBuilder;
use calamine::{Reader, open_workbook, Xlsx, Data};
use serde_json::Value as JsonValue;

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

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum DataViewMode {
    #[default]
    SideBySide,
    Tabbed,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum TabbedDataView {
    #[default]
    Original,
    Generated,
}

struct DadagenApp {
    // File handling
    file_path: Option<PathBuf>,
    file_content: String,
    file_type: FileType,
    
    // Parsed data
    parsed_data: ParsedData,
    column_types: Vec<InferredType>,
    
    // DSL generation
    dsl_output: String,
    
    // Generated data
    generated_data: ParsedData,
    is_generating: bool,
    generation_error: Option<String>,
    
    // UI state
    data_view_mode: DataViewMode,
    show_settings: bool,
    show_about: bool,
    
    // Settings
    theme_dark: bool,
    max_preview_rows: usize,
    generate_row_count: usize,
    // Tab selection for tabbed data view
    tab_selected: TabbedDataView,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum FileType {
    #[default]
    Unknown,
    Csv,
    Json,
    Excel,
}

#[derive(Default, Clone, Debug)]
struct ParsedData {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    total_rows: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum InferredType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    Email,
    Url,
    Unknown,
}

impl Default for InferredType {
    fn default() -> Self {
        InferredType::Unknown
    }
}

impl DadagenApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_path: None,
            file_content: String::new(),
            file_type: FileType::Unknown,
            parsed_data: ParsedData::default(),
            column_types: Vec::new(),
            dsl_output: String::new(),
            generated_data: ParsedData::default(),
            is_generating: false,
            generation_error: None,
            data_view_mode: DataViewMode::SideBySide,
            show_settings: false,
            show_about: false,
            theme_dark: true,
            max_preview_rows: 100,
            generate_row_count: 100,
            tab_selected: TabbedDataView::Original,
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
                    ui.label("Data View Mode:");
                    ui.radio_value(&mut self.data_view_mode, DataViewMode::SideBySide, "📊 Side-by-Side");
                    ui.radio_value(&mut self.data_view_mode, DataViewMode::Tabbed, "📑 Tabbed");
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
        
        // Main unified layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top section: DSL Editor (35% of window height)
            let total_height = ui.available_height();
            let dsl_height = total_height * 0.35;
            
            ui.vertical(|ui| {
                // DSL Editor Section
                ui.group(|ui| {
                    ui.set_height(dsl_height);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading("🔧 DSL Configuration");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("📋 Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = self.dsl_output.clone());
                                }
                                if ui.button("💾 Save").clicked() {
                                    self.save_dsl();
                                }
                            });
                        });
                        
                        ui.separator();
                        
                        if self.dsl_output.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(dsl_height * 0.3);
                                ui.label(egui::RichText::new("No DSL generated yet").weak());
                                ui.label(egui::RichText::new("Open a file to automatically generate a DSL schema").weak());
                            });
                        } else {
                            egui::ScrollArea::vertical()
                                .id_salt("dsl_scroll")
                                .max_height(dsl_height - 60.0)
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut self.dsl_output)
                                            .code_editor()
                                            .desired_width(f32::INFINITY)
                                    );
                                });
                        }
                    });
                });
                
                ui.add_space(5.0);

                // Middle Section: Controls
                ui.horizontal(|ui| {
                    ui.heading("🚀 Generation Controls");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("💾 Export Data").clicked() {
                            self.export_generated_data();
                        }
                        
                        ui.separator();

                        if self.is_generating {
                            ui.spinner();
                            ui.label("Generating...");
                        } else {
                            ui.add(egui::DragValue::new(&mut self.generate_row_count)
                                .speed(10)
                                .range(1..=10000)
                                .prefix("Rows: "));
                            
                            let button = egui::Button::new("🎲 Generate Fake Data");
                            if ui.add_enabled(!self.dsl_output.is_empty(), button).clicked() {
                                self.generate_fake_data();
                            }
                        }
                    });
                });

                ui.add_space(5.0);
                
                // Bottom section: Data Views
                ui.group(|ui| {
                    let container_h = ui.available_height();
                    ui.set_height(container_h);
                    
                    ui.vertical(|ui| {
                        ui.set_height(container_h);
                        match self.data_view_mode {
                            DataViewMode::SideBySide => {
                                let avail_h = ui.available_height();
                                let avail_w = ui.available_width();
                                
                                ui.horizontal(|ui| {
                                    ui.set_height(avail_h);
                                    
                                    // Left: Original Data
                                    ui.vertical(|ui| {
                                        ui.set_width(avail_w * 0.5 - 4.0);
                                        ui.set_height(avail_h);
                                        self.show_original_data_panel(ui, avail_h);
                                    });
                                    
                                    ui.separator();
                                    
                                    // Right: Generated Data
                                    ui.vertical(|ui| {
                                        ui.set_width(ui.available_width());
                                        ui.set_height(avail_h);
                                        self.show_generated_data_panel(ui, avail_h);
                                    });
                                });
                            }
                            DataViewMode::Tabbed => {
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui.selectable_label(self.tab_selected == TabbedDataView::Original, "📄 Original Data").clicked() {
                                            self.tab_selected = TabbedDataView::Original;
                                        }
                                        if ui.selectable_label(self.tab_selected == TabbedDataView::Generated, "✨ Generated Data").clicked() {
                                            self.tab_selected = TabbedDataView::Generated;
                                        }
                                    });
                                    ui.separator();
                                    
                                    let avail_h = ui.available_height();
                                    match self.tab_selected {
                                        TabbedDataView::Original => {
                                            self.show_original_data_panel(ui, avail_h);
                                        }
                                        TabbedDataView::Generated => {
                                            self.show_generated_data_panel(ui, avail_h);
                                        }
                                    }
                                });
                            }
                        }
                    });
                });
            });
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
    // Panel implementations
    fn show_original_data_panel(&mut self, ui: &mut egui::Ui, panel_height: f32) {
        ui.vertical(|ui| {
            ui.set_height(panel_height);
            ui.horizontal(|ui| {
                ui.heading("📄 Original Data");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📁 Open File").clicked() {
                        self.open_file();
                    }
                });
            });
            
            ui.separator();
            
            if let Some(path) = &self.file_path {
                ui.label(egui::RichText::new(format!("File: {}", path.file_name().unwrap().to_string_lossy()))
                    .strong());
                ui.label(format!("Rows: {} | Columns: {}", 
                    self.parsed_data.total_rows,
                    self.parsed_data.headers.len()));
                ui.add_space(5.0);
            }
            
            // Data table
            if !self.parsed_data.headers.is_empty() {
                egui::ScrollArea::both()
                    .id_salt("original_data_scroll")
                    .auto_shrink([false; 2])
                    .min_scrolled_height(ui.available_height())
                    .show(ui, |ui| {
                        self.show_data_table(ui, &self.parsed_data, &self.column_types);
                    });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(panel_height * 0.3);
                    ui.label(egui::RichText::new("📁 No file loaded").size(18.0).weak());
                    ui.label(egui::RichText::new("Drag & drop a file or click 'Open File'").weak());
                    ui.label(egui::RichText::new("Supported: CSV, JSON, Excel").small().weak());
                });
            }
        });
    }
    
    fn show_generated_data_panel(&mut self, ui: &mut egui::Ui, panel_height: f32) {
        ui.vertical(|ui| {
            ui.set_height(panel_height);
            ui.horizontal(|ui| {
                ui.heading("✨ Generated Data");
            });
            
            ui.separator();
            
            // Show error if any
            if let Some(error) = &self.generation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", error));
                ui.add_space(5.0);
            }
            
            // Show generated data
            if !self.generated_data.headers.is_empty() {
                ui.label(format!("Generated {} rows with {} columns", 
                    self.generated_data.rows.len(),
                    self.generated_data.headers.len()));
                ui.add_space(5.0);
                
                egui::ScrollArea::both()
                    .id_salt("generated_data_scroll")
                    .auto_shrink([false; 2])
                    .min_scrolled_height(ui.available_height())
                    .show(ui, |ui| {
                        // Use empty types vec since we don't need type annotations for generated data
                        let empty_types = vec![];
                        self.show_data_table(ui, &self.generated_data, &empty_types);
                    });
            } else if !self.is_generating && self.generation_error.is_none() {
                ui.vertical_centered(|ui| {
                    ui.add_space(panel_height * 0.3);
                    ui.label(egui::RichText::new("✨ No data generated yet").size(18.0).weak());
                    if self.dsl_output.is_empty() {
                        ui.label(egui::RichText::new("Load a file to generate DSL first").weak());
                    } else {
                        ui.label(egui::RichText::new("Click 'Generate' to create fake data").weak());
                    }
                });
            }
        });
    }
    
    fn show_data_table(&self, ui: &mut egui::Ui, data: &ParsedData, types: &[InferredType]) {
        use egui_extras::{TableBuilder, Column};
        
        if data.headers.is_empty() {
            return;
        }

        ui.push_id(if types.is_empty() { "gen_table" } else { "orig_table" }, |ui| {
            let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .columns(Column::auto(), data.headers.len())
                    .min_scrolled_height(0.0)
                    .vscroll(false); // Let the outer ScrollArea handle scrolling
                            
            table.header(25.0, |mut header| {
                    for (idx, col_name) in data.headers.iter().enumerate() {
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.strong(col_name);
                                // Show inferred type if available
                                if let Some(col_type) = types.get(idx) {
                                    let type_str = match col_type {
                                        InferredType::String => "String",
                                        InferredType::Integer => "Integer",
                                        InferredType::Float => "Float",
                                        InferredType::Boolean => "Boolean",
                                        InferredType::Date => "Date",
                                        InferredType::Email => "Email",
                                        InferredType::Url => "URL",
                                        InferredType::Unknown => "Unknown",
                                    };
                                    ui.small(format!("({})", type_str));
                                }
                            });
                        });
                    }
                }).body(|body| {
                    body.rows(20.0, data.rows.len(), |mut row| {
                        let row_index = row.index();
                        if let Some(data_row) = data.rows.get(row_index) {
                            for cell in data_row {
                                row.col(|ui| {
                                    ui.label(cell);
                                });
                            }
                        }
                    });
                });
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
        
        // Parse the file based on type
        let parse_result = match self.file_type {
            FileType::Csv => self.parse_csv(&path),
            FileType::Json => self.parse_json(&path),
            FileType::Excel => self.parse_excel(&path),
            FileType::Unknown => {
                // Try to read as text for preview
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        self.file_content = if content.len() > 10000 {
                            format!("{}...\n\n[File truncated, showing first 10KB]", &content[..10000])
                        } else {
                            content
                        };
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to read file: {}", e)),
                }
            }
        };
        
        match parse_result {
            Ok(_) => {
                self.file_path = Some(path);
                // Infer column types and generate DSL if we have headers
                self.column_types = self.infer_column_types();
                // Generate DSL from parsed data
                self.generate_dsl();
                tracing::info!("File loaded and parsed successfully");
            }
            Err(e) => {
                tracing::error!("Error loading file: {}", e);
                // Show error in file content preview
                self.file_content = format!("Error loading file: {}", e);
            }
        }
    }
    
    fn generate_dsl(&mut self) {
        if self.parsed_data.headers.is_empty() {
            return;
        }
        
        let file_name = self.file_path.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|n| n.to_str())
            .unwrap_or("data");
        
        let mut dsl = String::new();
        
        // Header
        dsl.push_str(&format!(
            "# Dadagen DSL Configuration\n\
             # Auto-generated from: {}\n\
             # Date: {}\n\
             # Total rows in source: {}\n\n",
            self.file_path.as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            self.parsed_data.total_rows
        ));
        
        // Schema definition
        dsl.push_str(&format!("schema {} {{\n", file_name));
        
        // Generate field definitions based on inferred types
        for (idx, header) in self.parsed_data.headers.iter().enumerate() {
            let field_name = header
                .replace(" ", "_")
                .replace("-", "_")
                .to_lowercase();
            
            let field_def = if let Some(col_type) = self.column_types.get(idx) {
                match col_type {
                    InferredType::Integer => {
                        // Sample min/max from data
                        let (min, max) = self.get_number_range(idx);
                        format!("  \"{}\": number(min={}, max={}),", 
                            field_name, min.unwrap_or(0), max.unwrap_or(1000))
                    }
                    InferredType::Float => {
                        let (min, max) = self.get_number_range(idx);
                        format!("  \"{}\": number(min={}, max={}),", 
                            field_name, min.unwrap_or(0), max.unwrap_or(1000))
                    }
                    InferredType::Boolean => {
                        format!("  \"{}\": boolean,", field_name)
                    }
                    InferredType::Email => {
                        format!("  \"{}\": regexgen \"[a-z]{{5,10}}@[a-z]{{3,8}}\\.com\",", field_name)
                    }
                    InferredType::Url => {
                        format!("  \"{}\": template \"https://example.com/{{id}}\",", field_name)
                    }
                    InferredType::Date => {
                        format!("  \"{}\": regexgen \"20[0-2][0-9]-[0-1][0-9]-[0-3][0-9]\",", field_name)
                    }
                    InferredType::String | InferredType::Unknown => {
                        // Check if it could be from a list
                        let unique_values = self.get_unique_values(idx);
                        if unique_values.len() <= 10 && unique_values.len() > 1 {
                            // Looks like categorical data
                            let quoted_values: Vec<String> = unique_values.iter()
                                .map(|v| format!("\"{}\"", v))
                                .collect();
                            format!("  \"{}\": choice({}),", field_name, quoted_values.join(", "))
                        } else {
                            // Check average length for string pattern
                            let avg_len = self.get_average_string_length(idx);
                            format!("  \"{}\": regexgen \"[A-Za-z0-9 ]{{5,{}}}\",", 
                                field_name, avg_len.max(10))
                        }
                    }
                }
            } else {
                format!("  \"{}\": string,", field_name)
            };
            
            dsl.push_str(&field_def);
            dsl.push('\n');
        }
        
        dsl.push_str("}\n\n");
        dsl.push_str("# Usage:\n");
        dsl.push_str("#   dadagen generate <schema_file> --output data.csv --count 100 --format csv\n");
        dsl.push_str("#   dadagen generate <schema_file> --output data.json --count 100 --format json\n");
        
        self.dsl_output = dsl;
        tracing::info!("Generated DSL with {} fields", self.parsed_data.headers.len());
    }
    
    fn get_number_range(&self, col_idx: usize) -> (Option<i64>, Option<i64>) {
        let mut min: Option<i64> = None;
        let mut max: Option<i64> = None;
        
        for row in &self.parsed_data.rows {
            if let Some(val) = row.get(col_idx) {
                if let Ok(num) = val.parse::<i64>() {
                    min = Some(min.map(|m| m.min(num)).unwrap_or(num));
                    max = Some(max.map(|m| m.max(num)).unwrap_or(num));
                }
            }
        }
        
        (min, max)
    }
    
    fn get_unique_values(&self, col_idx: usize) -> Vec<String> {
        use std::collections::HashSet;
        
        let mut unique: HashSet<String> = HashSet::new();
        for row in &self.parsed_data.rows {
            if let Some(val) = row.get(col_idx) {
                if !val.is_empty() {
                    unique.insert(val.clone());
                }
            }
        }
        
        unique.into_iter().collect()
    }
    
    fn get_average_string_length(&self, col_idx: usize) -> usize {
        let mut total = 0;
        let mut count = 0;
        
        for row in &self.parsed_data.rows {
            if let Some(val) = row.get(col_idx) {
                total += val.len();
                count += 1;
            }
        }
        
        if count > 0 {
            (total / count).max(10)
        } else {
            20
        }
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
    
    fn generate_fake_data(&mut self) {
        use dadagen_core::{parser::parse_dsl, Context, generator_registry::GeneratorRegistry};
        
        self.is_generating = true;
        self.generation_error = None;
        self.generated_data = ParsedData::default();
        
        // Parse DSL
        let doc = match parse_dsl(&self.dsl_output) {
            Ok(doc) => doc,
            Err(e) => {
                self.generation_error = Some(format!("DSL parsing error: {}", e));
                self.is_generating = false;
                return;
            }
        };
        
        // Create generator registry
        let registry = GeneratorRegistry::new();
        
        // Generate data
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        
        // Extract field names for headers
        for field_def in &doc.fields {
            headers.push(field_def.name.clone());
        }
        
        // Use a single context and increment iteration once per row
        let mut context = Context::new();
        for _ in 0..self.generate_row_count {
            if let Err(e) = context.increment_iteration() {
                self.generation_error = Some(format!("Context error: {}", e));
                self.is_generating = false;
                return;
            }
            let mut row = Vec::new();
            for field_def in &doc.fields {
                match registry.create_from_ast(&field_def.generator) {
                    Ok(generator) => {
                        match generator.generate(&mut context) {
                            Ok(value) => row.push(value),
                            Err(e) => {
                                self.generation_error = Some(format!("Generation error for field '{}': {}", field_def.name, e));
                                self.is_generating = false;
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        self.generation_error = Some(format!("Generator creation error for field '{}': {}", field_def.name, e));
                        self.is_generating = false;
                        return;
                    }
                }
            }
            rows.push(row);
        }
        
        self.generated_data = ParsedData {
            headers,
            rows,
            total_rows: self.generate_row_count,
        };
        
        self.tab_selected = TabbedDataView::Generated;
        self.is_generating = false;
        tracing::info!("Generated {} rows of fake data", self.generate_row_count);
    }
    
    fn export_generated_data(&self) {
        if self.generated_data.headers.is_empty() {
            return;
        }
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV Files", &["csv"])
            .add_filter("JSON Files", &["json"])
            .set_file_name("generated_data.csv")
            .save_file()
        {
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("csv");
            
            let result = match extension {
                "json" => self.export_as_json(&path),
                _ => self.export_as_csv(&path),
            };
            
            match result {
                Ok(_) => tracing::info!("Data exported to: {}", path.display()),
                Err(e) => tracing::error!("Export error: {}", e),
            }
        }
    }
    
    fn export_as_csv(&self, path: &PathBuf) -> Result<()> {
        use csv::Writer;
        
        let mut writer = Writer::from_path(path)?;
        
        // Write headers
        writer.write_record(&self.generated_data.headers)?;
        
        // Write rows
        for row in &self.generated_data.rows {
            writer.write_record(row)?;
        }
        
        writer.flush()?;
        Ok(())
    }
    
    fn export_as_json(&self, path: &PathBuf) -> Result<()> {
        use serde_json::{json, Value};
        
        let mut records: Vec<Value> = Vec::new();
        
        for row in &self.generated_data.rows {
            let mut obj = serde_json::Map::new();
            for (idx, header) in self.generated_data.headers.iter().enumerate() {
                if let Some(value) = row.get(idx) {
                    obj.insert(header.clone(), json!(value));
                }
            }
            records.push(Value::Object(obj));
        }
        let json_output = serde_json::to_string_pretty(&records)?;
        std::fs::write(path, json_output)?;
        Ok(())
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
    
    // File parsing implementations
    fn parse_csv(&mut self, path: &PathBuf) -> Result<()> {
        let file_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read CSV file: {}", path.display()))?;
        
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(file_content.as_bytes());
        
        // Read headers
        let headers = reader.headers()
            .context("Failed to read CSV headers")?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<_>>();
        
        // Read rows (limit to max_preview_rows for performance)
        let mut rows = Vec::new();
        let mut total_rows = 0;
        
        for result in reader.records() {
            total_rows += 1;
            if rows.len() < self.max_preview_rows {
                let record = result.context("Failed to read CSV record")?;
                rows.push(record.iter().map(|s| s.to_string()).collect());
            }
        }
        
        self.parsed_data = ParsedData {
            headers,
            rows,
            total_rows,
        };
        
        // Update file content preview
        self.file_content = format!(
            "CSV file with {} columns and {} rows\nShowing first {} rows\n\nColumns: {}",
            self.parsed_data.headers.len(),
            total_rows,
            self.parsed_data.rows.len(),
            self.parsed_data.headers.join(", ")
        );
        
        Ok(())
    }
    
    fn parse_json(&mut self, path: &PathBuf) -> Result<()> {
        let file_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read JSON file: {}", path.display()))?;
        
        let json: JsonValue = serde_json::from_str(&file_content)
            .context("Failed to parse JSON")?;
        
        // Convert JSON to tabular format
        match json {
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(anyhow::anyhow!("JSON array is empty"));
                }
                
                // Extract headers from first object
                if let Some(JsonValue::Object(first_obj)) = arr.first() {
                    let headers: Vec<String> = first_obj.keys().cloned().collect();
                    
                    // Extract rows
                    let mut rows = Vec::new();
                    for (idx, item) in arr.iter().enumerate() {
                        if idx >= self.max_preview_rows {
                            break;
                        }
                        
                        if let JsonValue::Object(obj) = item {
                            let row: Vec<String> = headers.iter()
                                .map(|h| {
                                    obj.get(h)
                                        .map(|v| match v {
                                            JsonValue::String(s) => s.clone(),
                                            JsonValue::Number(n) => n.to_string(),
                                            JsonValue::Bool(b) => b.to_string(),
                                            JsonValue::Null => "null".to_string(),
                                            _ => serde_json::to_string(v).unwrap_or_default(),
                                        })
                                        .unwrap_or_default()
                                })
                                .collect();
                            rows.push(row);
                        }
                    }
                    
                    self.parsed_data = ParsedData {
                        headers,
                        rows,
                        total_rows: arr.len(),
                    };
                    
                    self.file_content = format!(
                        "JSON array with {} objects\nShowing first {} objects\n\nFields: {}",
                        arr.len(),
                        self.parsed_data.rows.len(),
                        self.parsed_data.headers.join(", ")
                    );
                } else {
                    return Err(anyhow::anyhow!("JSON array does not contain objects"));
                }
            }
            JsonValue::Object(_) => {
                return Err(anyhow::anyhow!("JSON object not supported yet. Please provide an array of objects."));
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported JSON format. Expected array of objects."));
            }
        }
        
        Ok(())
    }
    
    fn parse_excel(&mut self, path: &PathBuf) -> Result<()> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .with_context(|| format!("Failed to open Excel file: {}", path.display()))?;
        
        // Get first sheet
        let sheet_names = workbook.sheet_names();
        if sheet_names.is_empty() {
            return Err(anyhow::anyhow!("Excel file has no sheets"));
        }
        
        let sheet_name = sheet_names[0].clone();
        let range = workbook.worksheet_range(&sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;
        
        let mut rows_iter = range.rows();
        
        // Extract headers from first row
        let headers = if let Some(header_row) = rows_iter.next() {
            header_row.iter()
                .map(|cell| match cell {
                    Data::String(s) => s.clone(),
                    Data::Int(i) => i.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::Empty => "".to_string(),
                    Data::Error(e) => format!("Error: {:?}", e),
                    Data::DateTime(dt) => format!("{:?}", dt),
                    Data::DateTimeIso(s) => s.clone(),
                    Data::DurationIso(s) => s.clone(),
                })
                .collect::<Vec<_>>()
        } else {
            return Err(anyhow::anyhow!("Excel sheet is empty"));
        };
        
        // Extract data rows
        let mut rows = Vec::new();
        let mut total_rows = 0;
        
        for row in rows_iter {
            total_rows += 1;
            if rows.len() < self.max_preview_rows {
                let row_data: Vec<String> = row.iter()
                    .map(|cell| match cell {
                        Data::String(s) => s.clone(),
                        Data::Int(i) => i.to_string(),
                        Data::Float(f) => f.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::Empty => "".to_string(),
                        Data::Error(e) => format!("Error: {:?}", e),
                        Data::DateTime(dt) => format!("{:?}", dt),
                        Data::DateTimeIso(s) => s.clone(),
                        Data::DurationIso(s) => s.clone(),
                    })
                    .collect();
                rows.push(row_data);
            }
        }
        
        self.parsed_data = ParsedData {
            headers,
            rows,
            total_rows,
        };
        
        self.file_content = format!(
            "Excel file: {}\nColumns: {}, Rows: {}\nShowing first {} rows\n\nColumns: {}",
            sheet_name,
            self.parsed_data.headers.len(),
            total_rows,
            self.parsed_data.rows.len(),
            self.parsed_data.headers.join(", ")
        );
        
        Ok(())
    }
    
    // Type inference
    fn infer_column_types(&self) -> Vec<InferredType> {
        let mut types = vec![InferredType::Unknown; self.parsed_data.headers.len()];
        
        // Sample first N rows for type inference
        let sample_size = self.parsed_data.rows.len().min(100);
        
        for col_idx in 0..self.parsed_data.headers.len() {
            let mut sample_values: Vec<&str> = Vec::new();
            
            for row in self.parsed_data.rows.iter().take(sample_size) {
                if let Some(val) = row.get(col_idx) {
                    if !val.is_empty() {
                        sample_values.push(val.as_str());
                    }
                }
            }
            
            if sample_values.is_empty() {
                continue;
            }
            
            types[col_idx] = self.infer_type_from_samples(&sample_values);
        }
        
        types
    }
    
    fn infer_type_from_samples(&self, samples: &[&str]) -> InferredType {
        if samples.is_empty() {
            return InferredType::Unknown;
        }
        
        let mut all_integers = true;
        let mut all_floats = true;
        let mut all_bools = true;
        let mut all_emails = true;
        let mut all_urls = true;
        let mut all_dates = true;
        
        for &sample in samples {
            // Check integer
            if all_integers && sample.parse::<i64>().is_err() {
                all_integers = false;
            }
            
            // Check float
            if all_floats && sample.parse::<f64>().is_err() {
                all_floats = false;
            }
            
            // Check boolean
            if all_bools {
                let lower = sample.to_lowercase();
                if !matches!(lower.as_str(), "true" | "false" | "yes" | "no" | "1" | "0" | "t" | "f") {
                    all_bools = false;
                }
            }
            
            // Check email (simple check)
            if all_emails && !(sample.contains('@') && sample.contains('.')) {
                all_emails = false;
            }
            
            // Check URL (simple check)
            if all_urls && !sample.starts_with("http://") && !sample.starts_with("https://") {
                all_urls = false;
            }
            
            // Check date (simple patterns)
            if all_dates {
                let has_dash = sample.matches('-').count() >= 2;
                let has_slash = sample.matches('/').count() >= 2;
                if !has_dash && !has_slash {
                    all_dates = false;
                }
            }
        }
        
        // Return most specific type
        match () {
            _ if all_bools => InferredType::Boolean,
            _ if all_integers => InferredType::Integer,
            _ if all_floats => InferredType::Float,
            _ if all_emails => InferredType::Email,
            _ if all_urls => InferredType::Url,
            _ if all_dates => InferredType::Date,
            _ => InferredType::String,
        }
    }
}
