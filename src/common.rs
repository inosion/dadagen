use rand::Rng;
use std::collections::HashMap;

pub struct Context<T> {
    // Define the context fields here
    data_field_state: HashMap<String, T>,
}

impl<T> Context<T> {
    pub fn new() -> Self {
        Self {
            data_field_state: HashMap::new(),
        }
    }

    pub fn insert_data_field_state(&mut self, key: String, value: T) {
        self.data_field_state.insert(key, value);
    }

    pub fn get_data_field_state(&self, key: &str) -> Option<&T> {
        // self.data_field_state
        //     .get(key)
        //     .and_then(|value| value.as_ref())
        None
    }
}

/// All items, fields, objects, cells have an associated Generator
pub trait Generator<T, R: Rng> {
    fn name(&self) -> String;
    fn generate(&self, context: &mut Context<T>, dependant_list: &[String]) -> T {
        let result = self.internal_generate(context);
        // if dependant_list.contains(&self.name()) {
        //     context
        //         .data_field_state
        //         .insert(self.name().to_string(), result);
        // }
        result
    }
    fn internal_generate(&self, context: &Context<T>) -> T;
    fn dependencies(&self) -> &[String];
    fn random(&self) -> R;
}