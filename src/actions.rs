use std::process;

pub trait Action {
    fn isme(&self, s: &str) -> bool;
    fn run(&self, acts: &str);
}

pub struct ActQuit {}

impl Action for ActQuit {
    fn isme(&self, s: &str) -> bool {
        s.eq("quit") || s.eq("exit") || s.eq("q")
    }

    fn run(&self, _acts: &str) {
        process::exit(0);
    }
}
