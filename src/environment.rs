use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::{interpreter::{Value, RuntimeError}, token::Token};

pub struct Environment<'a> {
    enclosing: Option<Rc<RefCell<Environment<'a>>>>,
    values: HashMap<String, &'a Value>
}

impl<'a> Environment<'a> {
    pub fn new() -> Rc<RefCell<Environment<'a>>> {
        Rc::new(RefCell::new(Environment {
            enclosing: None,
            values: HashMap::new()
        }))
    }
    pub fn new_with_enclosing(enclosing: &Rc<RefCell<Environment<'a>>>) -> Rc<RefCell<Environment<'a>>> {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(enclosing.clone()),
            values: HashMap::new()
        }))
    }

    pub fn get(&self, name: &Token) -> Result<&'a Value, RuntimeError> {
        let value = self.values.get(&name.lexeme);

        match value {
            Some(value) => Ok(value),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name);
                }

                Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme)))
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &'a Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme)))
    }

    pub fn define(&mut self, name: String, value: &'a Value) {
        self.values.insert(name, value);
    }
}