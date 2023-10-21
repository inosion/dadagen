use rand::Rng;
use rand::rngs::ThreadRng;

mod common;
mod number_generation;
mod string_generation;
mod util;

use common::Generator;
use common::Context;

struct MyGenerator;

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
    let mut context = Context::new();

    // Example usage
    let generator = MyGenerator;

    let dependant_list: &[String] = &Vec::new();

    let generated_value = generator.generate(&mut context, dependant_list);

    println!("Generated value: {:?}", generated_value);
}