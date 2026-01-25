use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use clap::{Parser as ClapParser, Subcommand};
use dadagen_core::{parser::parse_dsl, Context, generator_registry::GeneratorRegistry};
use serde_json::json;

#[derive(Debug, ClapParser)]
#[command(name = "dadagen")]
#[command(about = "Random Data Generator", long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate data from a dadagen configuration file
    Generate {
        /// Path to the dadagen schema file (.dadagen)
        schema_file: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Number of records to generate
        #[arg(short = 'n', long, default_value = "10")]
        count: usize,
        
        /// Output format (csv, json, yaml)
        #[arg(short, long, default_value = "csv")]
        format: String,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    let args = Cli::parse();

    match args.cmd {
        Command::Generate { schema_file, output, count, format } => {
            if args.debug {
                println!("Generating {} records from schema: {:?}", count, schema_file);
                println!("Output format: {}", format);
            }
            
            // Read and parse the schema file
            let content = fs::read_to_string(&schema_file)
                .map_err(|e| anyhow::anyhow!("Failed to read schema file: {}", e))?;
            
            let doc = parse_dsl(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse DSL: {}", e))?;
            
            if args.debug {
                println!("Parsed {} fields", doc.fields.len());
            }
            
            // Create generator registry
            let registry = GeneratorRegistry::new();
            
            // Generate data
            let mut records = Vec::new();
            for i in 0..count {
                let mut context = Context::new();
                // Increment iteration counter for each record
                for _ in 0..i {
                    context.increment_iteration()?;
                }
                
                let mut record = serde_json::Map::new();
                for field_def in &doc.fields {
                    let generator = registry.create_from_ast(&field_def.generator)?;
                    let value = generator.generate(&mut context)?;
                    record.insert(field_def.name.clone(), json!(value));
                }
                records.push(record);
            }
            
            // Output data in requested format
            let output_str = match format.as_str() {
                "json" => {
                    serde_json::to_string_pretty(&records)?
                }
                "csv" => {
                    let mut csv_output = String::new();
                    
                    // Write headers
                    if let Some(first) = records.first() {
                        let headers: Vec<&str> = first.keys().map(|k| k.as_str()).collect();
                        csv_output.push_str(&headers.join(","));
                        csv_output.push('\n');
                        
                        // Write rows
                        for record in &records {
                            let values: Vec<String> = headers.iter()
                                .map(|h| {
                                    record.get(*h)
                                        .map(|v| match v {
                                            serde_json::Value::String(s) => format!("\"{}\"", s),
                                            _ => v.to_string(),
                                        })
                                        .unwrap_or_default()
                                })
                                .collect();
                            csv_output.push_str(&values.join(","));
                            csv_output.push('\n');
                        }
                    }
                    csv_output
                }
                _ => {
                    return Err(anyhow::anyhow!("Unsupported format: {}", format));
                }
            };
            
            // Write output
            if let Some(out_path) = output {
                fs::write(&out_path, output_str)?;
                if args.debug {
                    println!("Output written to: {:?}", out_path);
                }
            } else {
                print!("{}", output_str);
            }
        }
    }

    Ok(())
}
