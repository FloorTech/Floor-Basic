use std::collections::HashMap;
mod commands;

pub fn execute(line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    let cmd = parts[0];

    let mut valid_commands: HashMap<&str, fn(&[&str])> = HashMap::new();
    valid_commands.insert("SETVALUE", commands::SETVALUE);
    valid_commands.insert("PRINT", commands::PRINT);
    valid_commands.insert("PX", commands::PX);

    if let Some(action) = valid_commands.get(cmd) {
        action(&parts[1..]);
    } else {
        println!("Unknown command attempted! {}", cmd);
    }
}
