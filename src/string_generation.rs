use rand::Rng;
use crate::common::{Generator, Context};
use rand::rngs::ThreadRng;

struct GenderGenerator {
    name: String,
    style: String,
}

impl Generator<String, ThreadRng> for GenderGenerator {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn internal_generate(&self, _context: &Context<String>) -> String {
        match self.style.as_str() {
            "char" => {
                if rand::thread_rng().gen_range(0..2) == 0 {
                    "M".to_string()
                } else {
                    "F".to_string()
                }
            }
            "word" => {
                if rand::thread_rng().gen_range(0..2) == 0 {
                    "Male".to_string()
                } else {
                    "Female".to_string()
                }
            }
            _ => "Unknown".to_string(),
        }
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

impl GenderGenerator {
    fn new(name: String, style: String) -> Self {
        Self {
            name,
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gender_generator() {
        let dependant_list: &[String] = &Vec::new();

        let generator = GenderGenerator::new("gender".to_string(), "char".to_string());
        let mut context = Context::new();

        let generated_value = generator.generate(&mut context, dependant_list);
        assert!(generated_value == "M" || generated_value == "F");

        let generator = GenderGenerator::new("gender".to_string(), "word".to_string());
        let generated_value = generator.generate(&mut context, dependant_list);
        assert!(generated_value == "Male" || generated_value == "Female");
    }
}