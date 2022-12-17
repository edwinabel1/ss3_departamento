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
        let personal = &mut self
            .departamento
            .entry(d.to_owned())
            .or_insert_with(Departamento::new)
            .personal;
        personal.push(p.to_owned());
        personal.sort_by_key(|name| name.to_lowercase());
    }

    pub fn get_departamento(&self, d: &str) -> Option<&Departamento> {
        self.departamento.get(d)
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

    pub fn get_personal(&self) -> &[String] {
        self.personal.as_slice()
    }
}
