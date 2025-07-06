use std::env;
use std::fs::File;
use std::io::{Read, Result};

mod executor;
mod handlers;

fn open_file(path: &str) -> Result<()> {
    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer[0] == 0x00 {
        handlers::handle_fbp(buffer)
    } else {
        handlers::handle_fbin(buffer)
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
    println!(
        "> Sorry! Repl mode is currently not being worked on. If you really need this mode, contribute to the GitHub."
    );
}
