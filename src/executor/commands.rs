use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref LATEST_STORED_VALUE: Mutex<String> = Mutex::new("undefined".to_string());
}

// OPCODE: 0x01
// Sets the currently-stored value, which can be used for commands like PRINT or PX.
pub fn SETVALUE(args: &[&str]) {
    let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
    *stored_value = args.join(" ").to_string();
}

// OPCODE: 0x02
// Prints the currently-stored value to the console
pub fn PRINT(_: &[&str]) {
    let stored_value = LATEST_STORED_VALUE.lock().unwrap();
    println!("[fBASIC] {}", *stored_value);
}

// OPCODE: 0x03
// Renders a single pixel using the colorname stored in the currently-stored value
pub fn PX(_: &[&str]) {
    let stored_value = LATEST_STORED_VALUE.lock().unwrap();
    let mut colors: HashMap<&str, &str> = HashMap::new();
    colors.insert("refresh", "\x1B[2J\x1B[H");
    colors.insert("newline", "\n");
    colors.insert("red", "🟥");
    colors.insert("orange", "🟧");
    colors.insert("yellow", "🟨");
    colors.insert("green", "🟩");
    colors.insert("blue", "🟦");
    colors.insert("purple", "🟪");
    colors.insert("brown", "🟫");
    colors.insert("black", "⬛");
    colors.insert("pink", "⬜");
    colors.insert("bricks", "🧱");

    if let Some(emoji) = colors.get(stored_value.as_str()) {
        print!("{}", emoji);
    } else {
        println!("Unknown color attempted to render pixel! {}", *stored_value);
    }
}

// OPCODE: 0x04
pub fn GOTO(_: &[&str]) {}
