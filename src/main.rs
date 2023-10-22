extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

use std::fs;
use std::path::PathBuf;

use rand::Rng;
use rand::rngs::ThreadRng;
use clap::{Parser as ClapParser, Subcommand};


mod common;
mod number_generation;
mod string_generation;
mod util;
mod dsl;

use common::Generator;
use common::Context;
use dsl::{DslParser, Rule};

struct MyGenerator;


#[derive(Debug, ClapParser)]
#[command(name = "dadagen")]
#[command(about = "Random Data Generator", long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    // optional because, default execution with no args will list all tasks/issues
    cmd: Command

}

#[derive(Subcommand, Debug)]
enum Command {
    /// Add a new issue to the default repository
    Generate {
        /// The title of your issue/task
        dadagen_config: PathBuf,
    },
}

impl Generator<i32, ThreadRng> for MyGenerator {
    fn name(&self) -> String {
        "example".to_string()
    }

    fn internal_generate(&self, _context: &Context<i32>) -> i32 {
        // Implement the generation logic here using the provided random number generator
        self.random().gen_range(0..100)
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

fn main() {
    let args = Cli::parse();

    let mut context = Context::new();

    // Example usage
    let generator = MyGenerator;

    let dependant_list: &[String] = &Vec::new();

    let generated_value = generator.generate(&mut context, dependant_list);

    println!("Generated value: {:?}", generated_value);
    match args.cmd {
        Command::Generate { dadagen_config } => {
            println!("Generating data from config: {:?}", dadagen_config);
            let x = fs::read_to_string(dadagen_config).expect("Something went wrong reading the file");
            let mut result = DslParser::parse(Rule::dsl, x.as_str()).unwrap();
        }
    }

}