# QB-COM

A Production-Ready QBasic/QuickBASIC 4.5 Compiler and Interpreter written in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

## Features

- **Full QBasic Language Support**: Complete implementation of QBasic/QuickBASIC 4.5 syntax and semantics
- **Interactive Interpreter (REPL)**: Run programs interactively with immediate feedback
- **Bytecode Compilation**: Compile to bytecode for faster execution
- **Native Compilation**: Compile to native executables using LLVM (future feature)
- **Hardware Emulation**: VGA graphics, PC speaker sound, DOS memory model
- **High Performance**: Optimized Rust implementation with modern compiler techniques
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (1.70 or later)

### Quick Setup

**Windows:**
```batch
setup.bat
```

**Linux/macOS:**
```bash
chmod +x setup.sh
./setup.sh
```

### Manual Build

```bash
# Build the project
cargo build --release

# Install globally
cargo install --path cli
```

## Usage

### Run a QBasic Program

```bash
# Using cargo
cargo run -- run examples/hello.bas

# Or if installed globally
qb run examples/hello.bas
```

### Interactive Mode (REPL)

```bash
cargo run -- repl
```

### Compile to Bytecode

```bash
cargo run -- build examples/hello.bas -o hello.qbc
```

### Check for Errors

```bash
cargo run -- check examples/hello.bas
```

### Create New Project

```bash
cargo run -- init myproject
cd myproject
```

## Example Programs

### Hello World
```basic
PRINT "Hello, World!"
END
```

### Simple Calculator
```basic
DIM a, b AS SINGLE
INPUT "Enter first number: ", a
INPUT "Enter second number: ", b
PRINT "Sum: "; a + b
PRINT "Difference: "; a - b
PRINT "Product: "; a * b
END
```

### Fibonacci Sequence
```basic
DIM n, i AS INTEGER
DIM a, b, c AS LONG
INPUT "How many numbers? ", n

a = 0
b = 1
FOR i = 1 TO n
    PRINT a
    c = a + b
    a = b
    b = c
NEXT i
END
```

## Architecture

QB-COM is organized as a Cargo workspace with the following crates:

### Binary Crate
- **cli**: Command-line interface and entry point

### Library Crates (`crates/`)
- **core**: Core types, memory emulation, and error handling
- **lexer**: Lexical analyzer (tokenizer)
- **parser**: Parser and Abstract Syntax Tree (AST)
- **semantic**: Semantic analysis and type checker
- **vm**: Bytecode compiler and Virtual Machine
- **codegen**: LLVM backend for native compilation
- **hal**: Hardware abstraction layer (DOS emulation)

## Supported Commands

| Command | Description |
|---------|-------------|
| `qb run <file>` | Run a QBasic program |
| `qb build <file>` | Compile to bytecode |
| `qb compile <file>` | Compile to native executable |
| `qb tokenize <file>` | Display token stream |
| `qb parse <file>` | Display AST |
| `qb check <file>` | Check for errors |
| `qb init <name>` | Create new project |
| `qb repl` | Interactive mode |
| `qb config` | Show configuration |

## Supported QBasic Features

### Data Types
- INTEGER (16-bit)
- LONG (32-bit)
- SINGLE (32-bit float)
- DOUBLE (64-bit float)
- STRING (variable length)
- Fixed-length strings
- User-defined types (TYPE...END TYPE)

### Control Structures
- IF...THEN...ELSE...END IF
- SELECT CASE...END SELECT
- FOR...NEXT
- WHILE...WEND
- DO...LOOP (WHILE/UNTIL)
- GOTO, GOSUB, RETURN

### Procedures
- SUB...END SUB
- FUNCTION...END FUNCTION
- DECLARE (external procedures)
- CALL
- Parameter passing (BYVAL/BYREF)

### I/O
- PRINT, INPUT, LINE INPUT
- File operations (OPEN, CLOSE, GET, PUT)
- Binary file access
- Random access files

### Graphics
- SCREEN (modes 0-13)
- PSET, PRESET
- LINE, CIRCLE, DRAW
- PAINT, VIEW, WINDOW
- COLOR, PALETTE, CLS

### Sound
- BEEP, SOUND
- PLAY (MML support)

### Memory
- PEEK, POKE
- DEF SEG
- VARPTR, VARSEG

## Configuration

Create a configuration file at:
- **Windows**: `%APPDATA%\qbc\QB-COM\config\config.toml`
- **Linux**: `~/.config/QB-COM/config.toml`
- **macOS**: `~/Library/Application Support/com.qbc.QB-COM/config.toml`

Example configuration:
```toml
[compiler]
optimization_level = 2
target = "native"

[runtime]
memory_limit_mb = 16
enable_graphics = true
enable_sound = true

[display]
screen_mode = 0
scale = 2.0
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Microsoft's QBasic and QuickBASIC 4.5
- Built with the Rust programming language
- Thanks to the QB64 and FreeBASIC communities for inspiration

## Repository

[https://github.com/thirawat27/QB-COM](https://github.com/thirawat27/QB-COM)
