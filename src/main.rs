use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

mod executor;

fn handle_fbin(buffer: Vec<u8>) -> Result<()> {
    let mut i: usize = 0;
    while i < buffer.len() {
        let opcode: u8 = buffer[i];
        i += 1;

        match opcode {
            // SETVALUE
            // Sets the currently-stored value, which can be used for commands like PRINT or PX.
            0x01 => {
                let mut arg_bytes: Vec<u8> = Vec::new();
                while i < buffer.len() && buffer[i] != 0x00 {
                    arg_bytes.push(buffer[i]);
                    i += 1;
                }

                i += 1;

                if let Ok(arg_str) = String::from_utf8(arg_bytes) {
                    executor::execute(&format!("SETVALUE {}", arg_str));
                } else {
                    println!("Invalid UTF-8 in SETVALUE arguments");
                }

                continue;
            }

            // PRINT
            // Prints the currently-stored value to the console.
            0x02 => executor::execute("PRINT"),

            // PX
            // Renders a single pixel using the colorname stored in the currently-stored value.
            0x03 => executor::execute("PX"),

            // GOTO
            // Moves execution to a line.
            0x04 => {
                let target: usize = buffer[i] as usize;
                i += 1;

                let adjusted: usize = target.saturating_sub(1);

                if adjusted < buffer.len() {
                    i = adjusted;
                    continue;
                } else {
                    println!("GOTO target {} out of bounds", target);
                    break;
                }
            }

            // INPUT
            // Takes text-based input from the console, then assigns the value to the current value.
            0x05 => executor::execute("INPUT"),

            // APPEND_VALUE
            // Appends a text-based value to the currently-stored value.
            0x06 => {
                let mut arg_bytes: Vec<u8> = Vec::new();
                while i < buffer.len() && buffer[i] != 0x00 {
                    arg_bytes.push(buffer[i]);
                    i += 1;
                }
                i += 1;
                if let Ok(arg_str) = String::from_utf8(arg_bytes) {
                    executor::execute(&format!("APPEND_VALUE {}", arg_str));
                }
                continue;
            }

            // CLEAR_VALUE
            // Clears the currently-stored value
            0x07 => executor::execute("CLEAR_VALUE"),

            // ADD
            // Adds a number-based value to the currently-stored value.
            0x08 => {
                let mut arg_bytes: Vec<u8> = Vec::new();
                while i < buffer.len() && buffer[i] != 0x00 {
                    arg_bytes.push(buffer[i]);
                    i += 1;
                }
                i += 1;
                if let Ok(arg_str) = String::from_utf8(arg_bytes) {
                    executor::execute(&format!("ADD {}", arg_str));
                }
                continue;
            }

            // SUB
            // Subtracts a number-based value from the currently-stored value.
            0x09 => {
                let mut arg_bytes: Vec<u8> = Vec::new();
                while i < buffer.len() && buffer[i] != 0x00 {
                    arg_bytes.push(buffer[i]);
                    i += 1;
                }
                i += 1;
                if let Ok(arg_str) = String::from_utf8(arg_bytes) {
                    executor::execute(&format!("SUB {}", arg_str));
                }
                continue;
            }

            // NL
            // Prints a new line to the console.
            0x0A => executor::execute("NL"),

            // CLS
            // Clears the console.
            0x0B => executor::execute("CLS"),

            // WAIT
            // Waits/sleeps a specific amount of milliseconds, based on the currently-stored value.
            0x0C => executor::execute("WAIT"),

            // Other(s)
            _ => {}
        }
    }

    Ok(())
}

fn handle_fbp(buffer: Vec<u8>) -> Result<()> {
    if buffer.len() < 6 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Buffer too small for metadata",
        ));
    }

    let format_marker: u8 = buffer[0];
    let version_number: u8 = buffer[1];

    if format_marker != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid format marker (expected 0x00)",
        ));
    }

    let name_len: usize = buffer[2] as usize;
    let name_end: usize = 3 + name_len;

    if buffer.len() < name_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Name exceeds buffer length",
        ));
    }

    let name = String::from_utf8_lossy(&buffer[3..name_end]);

    let desc_len_index: usize = name_end;
    let desc_len: usize = buffer[desc_len_index] as usize;
    let desc_start: usize = desc_len_index + 1;
    let desc_end: usize = desc_start + desc_len;

    if buffer.len() < desc_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Description exceeds buffer length",
        ));
    }

    let description = String::from_utf8_lossy(&buffer[desc_start..desc_end]);

    let fbin_len_start: usize = desc_end;

    if buffer.len() < fbin_len_start + 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough data for .fbin length",
        ));
    }

    let fbin_len: usize =
        u16::from_be_bytes([buffer[fbin_len_start], buffer[fbin_len_start + 1]]) as usize;

    let fbin_start: usize = fbin_len_start + 2;
    let fbin_end: usize = fbin_start + fbin_len;

    if buffer.len() < fbin_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            ".fbin data is incomplete",
        ));
    }

    let fbin_data: &[u8] = &buffer[fbin_start..fbin_end];
    println!("Package Name: {}", name);
    println!("Description: {}", description);
    println!("Version: {}", version_number);
    println!("Running embedded program...");
    return Ok(handle_fbin(fbin_data.to_vec())?);
}

fn open_file(path: &str) -> Result<()> {
    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer[0] == 0x00 {
        handle_fbp(buffer)
    } else {
        handle_fbin(buffer)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));

    if args.len() > 1 {
        println!("Using file mode...");
        open_file(args[1].as_str()).unwrap();
        return;
    }

    println!("Using repl mode...");
    println!(
        "> Sorry! Repl mode is currently not being worked on. If you really need this mode, contribute to the GitHub."
    );
}
