use std::collections::HashMap;

pub struct Empresa {
    departamento: HashMap<String, Departamento>,
}

impl Empresa {
    pub fn new() -> Empresa {
        Empresa {
            departamento: HashMap::new(),
        }
    }

    pub fn add(&mut self, p: &str, d: &str) {
        self.departamento
            .entry(d.to_owned())
            .or_insert_with(Departamento::new)
            .personal
            .push(p.to_owned());
    }

    pub fn get_departamento(&self) -> &HashMap<String, Departamento> {
        &self.departamento
    }
}

#[derive(Debug)]
pub struct Departamento {
    personal: Vec<String>,
}
impl Departamento {
    pub fn new() -> Departamento {
        Departamento {
            personal: Vec::new(),
        }
    }
}
