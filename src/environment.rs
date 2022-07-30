use std::collections::HashMap;

use crate::{interpreter::{Value, RuntimeError}, token::Token};

pub struct Environment<'a> {
    values: HashMap<String, &'a Value>
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn get(&self, name: &Token) -> Result<&'a Value, RuntimeError> {
        let value = self.values.get(&name.lexeme);

        match value {
            Some(value) => Ok(value),
            None => Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme)))
        }
    }

    pub fn assign(&mut self, name: &Token, value: &'a Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme)))
    }

    pub fn define(&mut self, name: String, value: &'a Value) {
        self.values.insert(name, value);
    }
}