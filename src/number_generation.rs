use rand::Rng;
use crate::common::{Generator, Context};
use rand::rngs::ThreadRng;

struct IntegerGenerator {
    name: String,
    min: i32,
    max: i32,
}

impl Generator<'_, i32, ThreadRng> for IntegerGenerator {
    fn name(&self) -> &str {
        &self.name
    }

    fn internal_generate(&self, _context: &Context) -> i32 {
        self.random().gen_range(self.min..=self.max)
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

impl IntegerGenerator {
    fn new(name: String, min: i32, max: i32) -> Self {
        Self {
            name,
            min,
            max,
        }
    }
}
struct DoubleGenerator {
    name: String,
    min: f64,
    max: f64,
    precision: i32,
}

impl Generator<'_, f64, ThreadRng> for DoubleGenerator {
    fn name(&self) -> &str {
        &self.name
    }

    fn internal_generate(&self, _context: &Context) -> f64 {
        let value = rand::thread_rng().gen_range(self.min..self.max);
        (value * 10.0_f64.powi(self.precision)).round() / 10.0_f64.powi(self.precision)
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

impl DoubleGenerator {
    fn new(name: String, min: f64, max: f64, precision: i32) -> Self {
        Self {
            name,
            min,
            max,
            precision,
        }
    }
}

struct LongGenerator {
    name: String,
    min: i64,
    max: i64,
}

impl Generator<'_, i64, ThreadRng> for LongGenerator {
    fn name(&self) -> &str {
        &self.name
    }

    fn internal_generate(&self, _context: &Context) -> i64 {
        rand::thread_rng().gen_range(self.min..=self.max)
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

impl LongGenerator {
    fn new(name: String, min: i64, max: i64) -> Self {
        Self {
            name,
            min,
            max,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_generator() {
        let mut context = Context::new();
        let generator = IntegerGenerator::new("number".to_string(),0, 100);

        let dependant_list: &[&str] = &Vec::new();

        let generated_value = generator.generate(&mut context, dependant_list);

        assert!(generated_value >= 0 && generated_value <= 100);
    }

    #[test]
    fn test_long_generator() {
        let mut context = Context::new();
        let generator = LongGenerator::new("number".to_string(), 0, 100);

        let dependant_list: &[&str] = &Vec::new();

        let generated_value = generator.generate(&mut context, dependant_list);

        assert!(generated_value >= 0 && generated_value <= 100);
    }

    #[test]
    fn test_double_generator() {
        let mut context = Context::new();
        let generator = DoubleGenerator::new("number".to_string(), 0.0, 100.0, 2);

        let dependant_list: &[&str] = &Vec::new();

        let generated_value = generator.generate(&mut context, dependant_list);

        assert!(generated_value >= 0.0 && generated_value <= 100.0);
    }    
}