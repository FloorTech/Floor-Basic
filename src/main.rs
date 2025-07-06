use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
mod executor;

fn handle_fbin(buffer: Vec<u8>) -> Result<()> {
    let mut i = 0;
    while i < buffer.len() {
        let opcode = buffer[i];
        i += 1;

        match opcode {
            // SETVALUE
            0x01 => {
                let mut arg_bytes = Vec::new();
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
            }

            // PRINT
            0x02 => executor::execute("PRINT"),

            // PX
            0x03 => executor::execute("PX"),

            // Other(s)
            other => println!("Unknown opcode! 0x{:02X}", other),
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

    let format_marker = buffer[0];
    let version_number = buffer[1];

    if format_marker != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid format marker (expected 0x00)",
        ));
    }

    let name_len = buffer[2] as usize;
    let name_end = 3 + name_len;

    if buffer.len() < name_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Name exceeds buffer length",
        ));
    }

    let name = String::from_utf8_lossy(&buffer[3..name_end]);

    let desc_len_index = name_end;
    let desc_len = buffer[desc_len_index] as usize;
    let desc_start = desc_len_index + 1;
    let desc_end = desc_start + desc_len;

    if buffer.len() < desc_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Description exceeds buffer length",
        ));
    }

    let description = String::from_utf8_lossy(&buffer[desc_start..desc_end]);

    let fbin_len_start = desc_end;

    if buffer.len() < fbin_len_start + 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough data for .fbin length",
        ));
    }

    let fbin_len =
        u16::from_be_bytes([buffer[fbin_len_start], buffer[fbin_len_start + 1]]) as usize;

    let fbin_start = fbin_len_start + 2;
    let fbin_end = fbin_start + fbin_len;

    if buffer.len() < fbin_end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            ".fbin data is incomplete",
        ));
    }

    let fbin_data = &buffer[fbin_start..fbin_end];
    println!("Package Name: {}", name);
    println!("Description: {}", description);
    println!("Version: {}", version_number);
    println!("Running embedded program...");
    return Ok(handle_fbin(fbin_data.to_vec())?);
}

fn open_file(path: &str) -> Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer[0] == 0x00 {
        handle_fbp(buffer)
    } else {
        handle_fbin(buffer)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Using file mode...");
        open_file(args[1].as_str()).unwrap();
        return;
    }

    println!("Using repl mode...");
    println!("> Sorry! Repl mode is currently not being worked on. If you really need this mode, contribute to the GitHub.");
}
