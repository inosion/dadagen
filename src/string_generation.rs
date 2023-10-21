use rand::Rng;
use crate::common::{Generator, Context};
use rand::rngs::ThreadRng;

struct GenderGenerator<'a> {
    name: &'a str,
    style: &'a str,
}

impl<'a> Generator<'a, String, ThreadRng> for GenderGenerator<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn internal_generate(&self, _context: &Context) -> String {
        match self.style {
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

impl<'a> GenderGenerator<'a> {
    fn new(name: &'a str, style: &'a str) -> Self {
        Self {
            name,
            style,
        }
    }
}
