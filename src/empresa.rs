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

    pub fn get_all_personal(&self) -> Vec<(&String, &String)> {
        let mut r: Vec<(&String, &String)> = self
            .departamento
            .iter()
            .flat_map(|i| {
                let p = i.1.get_personal();
                p.iter().zip(vec![i.0; p.len()])
            })
            .collect();
        r.sort_by_key(|name| name.0.to_lowercase());
        r
    }
}
