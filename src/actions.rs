use crate::empresa::Empresa;
use std::{collections::HashMap, process};

pub fn act_init() -> HashMap<&'static str, Box<dyn Action>> {
    let mut r: HashMap<&str, Box<dyn Action>> = HashMap::new();
    r.insert("q", Box::new(ActQuit {}));
    r.insert("quit", Box::new(ActQuit {}));
    r.insert("exit", Box::new(ActQuit {}));
    r.insert("add", Box::new(ActAdd {}));
    r.insert("list", Box::new(ActList {}));
    r.insert("ls", Box::new(ActList {}));
    r
}

pub trait Action {
    fn run(&self, acts: &str, emp: &mut Empresa);
}

pub struct ActQuit {}
impl Action for ActQuit {
    fn run(&self, _acts: &str, _emp: &mut Empresa) {
        process::exit(0);
    }
}

pub struct ActAdd {}
impl Action for ActAdd {
    fn run(&self, acts: &str, emp: &mut Empresa) {
        println!("action[add].");
        let (p, d) = scan_fmt_some!(acts, "add {} to {}\r\n", String, String);
        if let Some((p, d)) = p.zip(d) {
            emp.add(&p, &d);
            println!("okay.");
        }
    }
}

pub struct ActList {}
impl Action for ActList {
    fn run(&self, acts: &str, emp: &mut Empresa) {
        println!("action[list].");
        if let Some(d) = acts.split(' ').nth(1) {
            let d = d.trim();
            println!("list departamento {}:", d);
            println!("{:#?}", emp.get_departamento(d).map(|d| d.get_personal()));
        } else {
            println!("list all departamento.");
            println!("{:#?}", emp.get_all_personal());
        }
    }
}
