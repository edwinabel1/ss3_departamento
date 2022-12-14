mod actions;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use actions::ActQuit;

fn main() {
    let my_actions = act_init();
    loop {
        print!("^_-> ");
        let mut act_str = String::new();

        io::stdout().flush().expect("flush()");
        if let Err(e) = io::stdin().read_line(&mut act_str) {
            println!("invalidate input : {}", e);
            continue;
        };

        let act_strs: Vec<&str> = act_str.split(' ').collect();
        let mut act_strs_iter = act_strs.iter();
        if let Some(s) = act_strs_iter.next() {
            if let Some(act) = my_actions.get(s.trim()) {
                act.run(&act_str)
            }
        }
    }
}

fn act_init() -> HashMap<&'static str, Box<dyn actions::Action>> {
    let mut r: HashMap<&str, Box<dyn actions::Action>> = HashMap::new();
    r.insert("q", Box::new(ActQuit {}));
    r.insert("quit", Box::new(ActQuit {}));
    r.insert("exit", Box::new(ActQuit {}));
    r
}
