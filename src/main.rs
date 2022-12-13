mod actions;
use std::io::{self, Write};

use actions::ActQuit;

fn main() {
    let my_actions = act_init();
    loop {
        print!("^_-> ");
        io::stdout().flush().expect("flush()");
        let mut act_str = String::new();

        if let Err(e) = io::stdin().read_line(&mut act_str) {
            println!("invalidate input : {}", e);
            continue;
        };
        let act_strs: Vec<&str> = act_str.split(' ').collect();
        let mut act_strs_iter = act_strs.iter();
        match act_strs_iter.next() {
            None => continue,
            Some(s) => my_actions.iter().for_each(|act| {
                if act.isme(s) {
                    act.run(&act_str);
                }
            }),
        }
    }
}

fn act_init() -> Vec<Box<dyn actions::Action>> {
    let my_actions: Vec<Box<dyn actions::Action>> = vec![Box::new(ActQuit {})];
    my_actions
}
