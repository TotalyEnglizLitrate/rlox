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

    pub fn get(&self, name: Box<str>) -> Result<Literal, Error> {
        if let Some(val) = self.0.get(&name) {
            Ok(val.clone())
        } else {
            Err(Error::new(
                0,
                "".into(),
                format!("Name {} not found in scope", name),
                RuntimeError,
            ))
        }
    }
}
