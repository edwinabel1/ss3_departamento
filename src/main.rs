mod actions;
use std::io;

fn main() {
    let my_actions: Vec<Box<dyn actions::Action>> = Vec::new();
    loop {
        print!("^");
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
