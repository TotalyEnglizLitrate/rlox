use std::collections::HashMap;

use crate::{
    error::{Error, ErrorKind::RuntimeError},
    tokens::Literal,
};

pub struct Environment{
    enclosing: Option<Box<Environment>>,
    map: HashMap<Box<str>, Literal>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Environment {
            enclosing: match enclosing {
                Some(env) => Some(Box::new(env)),
                None => None
            },
            map: HashMap::new()
        }
    }

    pub fn define(&mut self, name: Box<str>, value: Literal) {
        self.map.insert(name, value);
    }

    pub fn assign(&mut self, name: Box<str>, value: Literal, line: usize) -> Result<Literal, Error> {
        if self.map.contains_key(&name) {
            self.map.insert(name, value.clone());
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
        if let Some(val) = self.map.get(&name) {
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
