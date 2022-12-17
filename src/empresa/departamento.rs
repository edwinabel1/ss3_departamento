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

    pub fn get_mut_personal(&mut self) -> &mut Vec<String> {
        &mut self.personal
    }

    pub fn get_personal(&self) -> &Vec<String> {
        &self.personal
    }
}
