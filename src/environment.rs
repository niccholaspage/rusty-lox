use std::collections::HashMap;

use crate::{interpreter::{Value, RuntimeError}, token::Token};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        let value = self.values.get(&name.lexeme);

        match value {
            Some(value) => Ok(value),
            None => Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme)))
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}