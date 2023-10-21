use rand::Rng;
use rand::rngs::ThreadRng;

mod common;
mod number_generation;
mod string_generation;

use common::Generator;
use common::Context;

struct MyGenerator;

impl Generator<'_, i32, ThreadRng> for MyGenerator {
    fn name(&self) -> &str {
        "example"
    }

    fn internal_generate(&self, context: &Context) -> i32 {
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

    let dependant_list: &[&str] = &Vec::new();

    let generated_value = generator.generate(&mut context, dependant_list);

    println!("Generated value: {:?}", generated_value);
}