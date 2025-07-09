# Floor Basic

## About

fBASIC (Floor Basic) is a deliberately minimal—and delightfully annoying—bytecode language and VM, written in Rust, with companion Python tools to compile and package your `.fb` scripts into raw bytecode (`.fbin`) or metadata‑wrapped packages (`.fbp`). But the compiler and packager are closed-source, for your inconvenience. The language features:

- A global operand register via `SETVALUE`
- Simple drawing via `PX`
- Console logging via `PRINT`
- Absolute jumps via `GOTO`
- Text-based console inpit via `INPUT`
- Currently-stored value appending via `APPEND_VALUE`
- Clearing the currently-stored value via `CLEAR_VALUE`
- Adding to currently-stored value as number via `ADD`
- Subtracting from currently-stored value as number via `SUB`
- New-line character writing via `NL`
- Console-clearing via `CLS`
- Sleeping/waiting as milliseconds via `WAIT`
- Extensible opcode map (up to `0xFF`)
- Two formats:
  - **Raw** `.fbin` files (opcodes + data markers)
  - **Packaged** `.fbp` files (with metadata header)

---

## How to run fBASIC programs

### Prerequisites

- Operating System (linux and Windows are fully tested)
- Terminal
- Rust toolchain (nightly recommended)

### Build the VM

```bash
cd rust-fbasic
cargo build --release
```

This produces the interpreter binary at

```text
target/release/fbasic.exe # on Windows
target/release/fbasic # on Unix
```

### 1) Running raw `.fbin` bytecode

```bash
./target/release/fbasic path/to/program.fbin
```

### 2) Running packaged `.fbp` files

```bash
./target/release/fbasic path/to/program.fbp
```

The VM auto‑detects by looking for `0x00` as the first byte (metadata marker).

---

## How to compile and package

### Reverse-engineer the VM's file loading system

We do have our own closed-source compiler and packager, but that's no fun.

### Compile a `.fb` script into `.fbin`

```bash
python3 compiler.py
```

Reads `./program.fb` → writes `./program.fbin`.

### Wrap a `.fbin` into a `.fbp` package

```bash
python3 packager.py
```

Reads `./program.fbin` → writes `./program.fbp`.

Edit the metadata (package name, description) inside the `program.package.env` as needed.

---

## Project Layout

```text
.
├── LICENSE               # MIT License
├── README.md             # This file
├── Cargo.toml            # Cargo project metadata
└── src/                  # Source code root
    ├── main.rs
    └── executor/
        ├── mod.rs
        └── commands.rs
```

---

## How to contribute

1. Fork the repository and create your feature branch:
   ```bash
   git checkout -b feature/AnnoyingNewOpcode
   ```
3. Commit your changes:
   ```bash
   git commit -am "Add DELAY opcode (0x05)"
   ```
5. Push to your fork and open a Pull Request.
6. Make sure your code passes existing tests (if any), and update `README.md` if you add new commands or change formats.

This project is released under the [MIT License](./LICENSE). Feel free to use, modify, or spite your fellow developers!
