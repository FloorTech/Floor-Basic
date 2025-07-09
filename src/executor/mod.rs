use std::collections::HashMap;
mod commands;

pub fn execute(line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    let cmd = parts[0];

    let mut valid_commands: HashMap<&str, fn(&[&str])> = HashMap::new();
    valid_commands.insert("SETVALUE", commands::SETVALUE);
    valid_commands.insert("PRINT", commands::PRINT);
    valid_commands.insert("PX", commands::PX);
    valid_commands.insert("INPUT", commands::INPUT);
    valid_commands.insert("APPEND_VALUE", commands::APPEND_VALUE);
    valid_commands.insert("CLEAR_VALUE", commands::CLEAR_VALUE);
    valid_commands.insert("ADD", commands::ADD);
    valid_commands.insert("SUB", commands::SUB);
    valid_commands.insert("NL", commands::NL);
    valid_commands.insert("CLS", commands::CLS);
    valid_commands.insert("WAIT", commands::WAIT);

    if let Some(action) = valid_commands.get(cmd) {
        action(&parts[1..]);
    } else {
        println!("Unknown command attempted! {}", cmd);
    }
}
