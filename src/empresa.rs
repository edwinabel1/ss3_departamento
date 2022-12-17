mod departamento;
use departamento::Departamento;
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
            .get_mut_personal();
        personal.push(p.to_owned());
        personal.sort_by_key(|name| name.to_lowercase());
    }

    pub fn get_departamento(&self, d: &str) -> Option<&Departamento> {
        self.departamento.get(d)
    }
}
