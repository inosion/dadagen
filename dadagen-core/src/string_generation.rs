use rand::Rng;
use regex::Regex;
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



#[derive(Debug)]
pub struct TemplateGenerator {
    name: String,
    template: String,
    dependencies: Vec<String>,
}

impl TemplateGenerator {
    pub fn new(name: String, template: String) -> Self {
        let var_regex = Regex::new(r#"(\$\{(\w+)\})"#).unwrap();
        let dependencies = var_regex.find_iter(&template)
            .map(|m| m.as_str().trim_start_matches("${").trim_end_matches("}").to_string())
            .collect();

        Self {
            name,
            template,
            dependencies,
        }
    }
}

impl Generator<String, ThreadRng> for TemplateGenerator {

    fn name(&self) -> String {
        self.name.clone()
    }

    fn internal_generate(&self, _context: &Context<String>) -> String {
        let var_regex = Regex::new(r#"(\$\{(\w+)\})"#).unwrap();
        var_regex.replace_all(&self.template, |caps: &regex::Captures| {
            let field_name = caps.get(2).unwrap().as_str();
            // context.data_field_state(field_name).to_string()
            field_name.to_string()
        }).to_string()
    }

    fn dependencies(&self) -> &[String] {
        &[]
    }

    fn random(&self) -> ThreadRng {
        rand::thread_rng()
    }
}

impl Described for TemplateGenerator {
    fn get_description(&self) -> String {
        "Template Variable. Use ${..} names for other fields to build up a custom value".to_string()
    }
}

trait Described {
    fn get_description(&self) -> String;
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


