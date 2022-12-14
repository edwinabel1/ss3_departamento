use std::process;

pub trait Action {
    fn run(&self, acts: &str);
}

pub struct ActQuit {}

impl Action for ActQuit {
    fn run(&self, _acts: &str) {
        process::exit(0);
    }
}
