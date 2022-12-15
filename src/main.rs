mod actions;
mod empresa;

use std::io::{self, Write};
#[macro_use]
extern crate scan_fmt;
fn main() {
    let my_actions = actions::act_init();
    let mut empresa = empresa::Empresa::new();

    loop {
        print!("^_-> ");
        let mut act_str = String::new();

        io::stdout().flush().expect("flush()");
        io::stdin().read_line(&mut act_str).expect("read_line()");

        let s = act_str.split(' ').next();
        if let Some(act) = s.and_then(|ss| my_actions.get(ss.trim())) {
            act.run(&act_str, &mut empresa)
        }
    }
}
