use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

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
// Prints the currently-stored value to the console.
pub fn PRINT(_: &[&str]) {
    let stored_value = LATEST_STORED_VALUE.lock().unwrap();
    println!("[fBASIC] {}", *stored_value);
}

// OPCODE: 0x03
// Renders a single pixel using the colorname stored in the currently-stored value.
pub fn PX(_: &[&str]) {
    let stored_value = LATEST_STORED_VALUE.lock().unwrap();
    let mut colors: HashMap<&str, &str> = HashMap::new();
    colors.insert("newline", "\n");
    colors.insert("clear", " ");
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
        io::stdout().flush().unwrap();
    } else {
        println!("Unknown color attempted to render pixel! {}", *stored_value);
    }
}

// OPCODE: 0x04
// Moves execution to a line.
pub fn GOTO(_: &[&str]) {
    // This command is handled directly, as some values cannot be passed here.
}

// OPCODE: 0x05
// Takes text-based input from the console, then assigns the value to the current value.
pub fn INPUT(_: &[&str]) {
    let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
    let mut input: String = String::new();

    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read your input!");
    *stored_value = input.trim().to_string();
}

// OPCODE: 0x06
// Appends a text-based value to the currently-stored value.
pub fn APPEND_VALUE(args: &[&str]) {
    let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
    stored_value.push_str(&args.join(" "));
}

// OPCODE: 0x07
// Clears the currently-stored value
pub fn CLEAR_VALUE(_: &[&str]) {
    let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
    *stored_value = "undefined".to_string();
}

// OPCODE: 0x08
// Adds a number-based value to the currently-stored value.
pub fn ADD(args: &[&str]) {
    if let Ok(addend) = args.join(" ").parse::<i32>() {
        let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
        if let Ok(current) = stored_value.parse::<i32>() {
            *stored_value = (current + addend).to_string();
        }
    }
}

// OPCODE: 0x09
// Subtracts a number-based value from the currently-stored value.
pub fn SUB(args: &[&str]) {
    if let Ok(subtrahend) = args.join(" ").parse::<i32>() {
        let mut stored_value = LATEST_STORED_VALUE.lock().unwrap();
        if let Ok(current) = stored_value.parse::<i32>() {
            *stored_value = (current - subtrahend).to_string();
        }
    }
}

// OPCODE: 0x0A
// Prints a new line to the console.
pub fn NL(_: &[&str]) {
    println!();
}

// OPCODE: 0x0B
// Clears the console.
pub fn CLS(_: &[&str]) {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

// OPCODE: 0x0C
// Waits/sleeps a specific amount of milliseconds, based on the currently-stored value.
pub fn WAIT(_: &[&str]) {
    let stored_value = LATEST_STORED_VALUE.lock().unwrap();
    if let Ok(ms) = stored_value.parse::<u64>() {
        thread::sleep(Duration::from_millis(ms));
    } else {
        println!(
            "When running the wait command, the currently-stored value must be in milliseconds!"
        );
    }
}
