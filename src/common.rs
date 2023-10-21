use rand::Rng;
use std::collections::HashMap;

pub struct Context {
    // Define the context fields here
    data_field_state: HashMap<String, Box<dyn std::any::Any + 'static>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data_field_state: HashMap::new(),
        }
    }

    pub fn insert_data_field_state<T: 'static>(&mut self, key: String, value: T) {
        self.data_field_state.insert(key, Box::new(value));
    }

    pub fn get_data_field_state<T: 'static>(&self, key: &str) -> Option<&T> {
        self.data_field_state
            .get(key)
            .and_then(|value| value.downcast_ref())
    }
}

/// All items, fields, objects, cells have an associated Generator
pub trait Generator<'a, T: 'a + Copy + 'static, R: Rng + 'static> {
    fn name(&self) -> &str;
    fn generate(&self, context: &mut Context, dependant_list: &[&str]) -> T {
        let mut rng = self.random();
        let result = self.internal_generate(context);
        if dependant_list.contains(&self.name()) {
            context
                .data_field_state
                .insert(self.name().to_string(), Box::new(result));
        }
        result
    }
    fn internal_generate(&self, context: &Context) -> T;
    fn dependencies(&self) -> &[String];
    fn random(&self) -> R;
}