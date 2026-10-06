use std::collections::HashMap;

use crate::{
    error::{Error, ErrorKind::RuntimeError},
    tokens::Literal,
};

pub struct Environment(HashMap<Box<str>, Literal>);

impl Environment {
    pub fn new() -> Self {
        Environment(HashMap::new())
    }

    pub fn define(&mut self, name: Box<str>, value: Literal) {
        self.0.insert(name, value);
    }

    pub fn assign(&mut self, name: Box<str>, value: Literal, line: usize) -> Result<Literal, Error> {
        if self.0.contains_key(&name) {
            self.0.insert(name, value.clone());
            Ok(value)
        } else {
            Err(Error::new(
                line,
                "".into(),
                format!("Undefined variable {}", name),
                RuntimeError,
            ))
        }
    }

    pub fn get(&self, name: Box<str>, line: usize) -> Result<Literal, Error> {
        if let Some(val) = self.0.get(&name) {
            Ok(val.clone())
        } else {
            Err(Error::new(
                line,
                "".into(),
                format!("Undefined variable {}", name),
                RuntimeError,
            ))
        }
    }
}
