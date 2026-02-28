mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process;

use config::Config;
// use qb_core::errors::QError;
use qb_lexer::tokenize;
use qb_parser::parse;
use qb_semantic::analyze;
use qb_vm::{compile, run};

/// QB-COM: QBasic Compiler and Interpreter
#[derive(Parser)]
#[command(name = "qb")]
#[command(about = "A Production-Ready QBasic/QuickBASIC 4.5 Compiler")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a QBasic program in interpreter mode
    Run {
        /// Path to the QBasic source file
        file: PathBuf,
        
        /// Command line arguments to pass to the program
        args: Vec<String>,
    },
    
    /// Compile a QBasic program to bytecode
    Build {
        /// Path to the QBasic source file
        file: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Emit LLVM IR instead of bytecode
        #[arg(long)]
        llvm: bool,
        
        /// Emit bytecode file
        #[arg(long)]
        bytecode: bool,
    },
    
    /// Compile a QBasic program to native executable
    Compile {
        /// Path to the QBasic source file
        file: PathBuf,
        
        /// Output executable path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "2")]
        optimize: u8,
    },
    
    /// Tokenize a QBasic program and print tokens
    Tokenize {
        /// Path to the QBasic source file
        file: PathBuf,
    },
    
    /// Parse a QBasic program and print AST
    Parse {
        /// Path to the QBasic source file
        file: PathBuf,
    },
    
    /// Check a QBasic program for errors without running
    Check {
        /// Path to the QBasic source file
        file: PathBuf,
    },
    
    /// Initialize a new QBasic project
    Init {
        /// Project name
        name: String,
        
        /// Project directory (defaults to project name)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    /// Show configuration
    Config {
        /// Set a configuration value (key=value)
        #[arg(short, long)]
        set: Vec<String>,
    },
    
    /// Run REPL (Interactive mode)
    Repl,
}

fn main() {
    let cli = Cli::parse();
    
    // Load configuration
    let config = if let Some(config_path) = cli.config {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match toml::from_str(&content) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        eprintln!("Error parsing config file: {}", e);
                        Config::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading config file: {}", e);
                Config::default()
            }
        }
    } else {
        Config::load().unwrap_or_default()
    };
    
    if let Err(e) = run_command(cli.command, config, cli.verbose) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_command(command: Commands, config: Config, verbose: bool) -> Result<()> {
    match command {
        Commands::Run { file, args: _ } => {
            run_file(&file, config, verbose)
        }
        Commands::Build { file, output, llvm, bytecode } => {
            build_file(&file, output, config, verbose, llvm, bytecode)
        }
        Commands::Compile { file, output, optimize } => {
            compile_native(&file, output, optimize, config, verbose)
        }
        Commands::Tokenize { file } => {
            tokenize_file(&file)
        }
        Commands::Parse { file } => {
            parse_file(&file)
        }
        Commands::Check { file } => {
            check_file(&file)
        }
        Commands::Init { name, path } => {
            init_project(&name, path)
        }
        Commands::Config { set } => {
            if set.is_empty() {
                show_config(&config)
            } else {
                update_config(set)
            }
        }
        Commands::Repl => {
            run_repl()
        }
    }
}

fn run_file(file: &PathBuf, _config: Config, verbose: bool) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    if verbose {
        eprintln!("Tokenizing...");
    }
    let tokens = tokenize(&source)?;
    
    if verbose {
        eprintln!("Parsing...");
    }
    let ast = parse(tokens)?;
    
    if verbose {
        eprintln!("Analyzing...");
    }
    analyze(&ast)?;
    
    if verbose {
        eprintln!("Compiling to bytecode...");
    }
    let bytecode = compile(&ast)?;
    
    if verbose {
        eprintln!("Running...");
    }
    run(&bytecode)?;
    
    Ok(())
}

fn build_file(
    file: &PathBuf, 
    output: Option<PathBuf>, 
    _config: Config, 
    verbose: bool,
    _llvm: bool,
    _bytecode: bool
) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    if verbose {
        eprintln!("Tokenizing...");
    }
    let tokens = tokenize(&source)?;
    
    if verbose {
        eprintln!("Parsing...");
    }
    let ast = parse(tokens)?;
    
    if verbose {
        eprintln!("Analyzing...");
    }
    analyze(&ast)?;
    
    if verbose {
        eprintln!("Compiling to bytecode...");
    }
    let bytecode = compile(&ast)?;
    
    let output_path = output.unwrap_or_else(|| file.with_extension("qbc"));
    
    // Serialize bytecode
    let serialized = bincode::serialize(&bytecode)?;
    fs::write(&output_path, serialized)?;
    
    println!("Built: {}", output_path.display());
    
    Ok(())
}

fn compile_native(
    file: &PathBuf,
    output: Option<PathBuf>,
    optimize: u8,
    _config: Config,
    verbose: bool,
) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    if verbose {
        eprintln!("Tokenizing...");
    }
    let tokens = tokenize(&source)?;
    
    if verbose {
        eprintln!("Parsing...");
    }
    let ast = parse(tokens)?;
    
    if verbose {
        eprintln!("Analyzing...");
    }
    analyze(&ast)?;
    
    let output_path = output.unwrap_or_else(|| {
        if cfg!(windows) {
            file.with_extension("exe")
        } else {
            file.with_extension("")
        }
    });
    
    // Use native_codegen for LLVM backend
    if verbose {
        eprintln!("Compiling to native code (optimization level: {})...", optimize);
    }
    
    qb_codegen::compile_to_native(&ast, output_path.to_str().unwrap())?;
    
    println!("Compiled: {}", output_path.display());
    
    Ok(())
}

fn tokenize_file(file: &PathBuf) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    let tokens = tokenize(&source)?;
    
    for (i, token_info) in tokens.iter().enumerate() {
        println!("{:4}: {:?} (line {}, col {})", 
            i, 
            token_info.token, 
            token_info.line, 
            token_info.column
        );
    }
    
    Ok(())
}

fn parse_file(file: &PathBuf) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    let tokens = tokenize(&source)?;
    let ast = parse(tokens)?;
    
    println!("{:#?}", ast);
    
    Ok(())
}

fn check_file(file: &PathBuf) -> Result<()> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    let tokens = tokenize(&source)?;
    let ast = parse(tokens)?;
    analyze(&ast)?;
    
    println!("✓ No errors found!");
    
    Ok(())
}

fn init_project(name: &str, path: Option<PathBuf>) -> Result<()> {
    let project_dir = path.unwrap_or_else(|| PathBuf::from(name));
    
    fs::create_dir_all(&project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("examples"))?;
    
    // Create main.bas
    let main_bas = format!(r#"' {}
' A QBasic Program

PRINT "Hello, World!"

END
"#, name);
    fs::write(project_dir.join("src").join("main.bas"), main_bas)?;
    
    // Create example
    let example = r#"' Example program

PRINT "This is an example"
FOR i = 1 TO 10
    PRINT "Number:"; i
NEXT i
END
"#;
    fs::write(project_dir.join("examples").join("hello.bas"), example)?;
    
    // Create README
    let readme = format!(r#"# {}

A QBasic program.

## Running

```bash
qb run src/main.bas
```

## Building

```bash
qb build src/main.bas
```
"#, name);
    fs::write(project_dir.join("README.md"), readme)?;
    
    println!("✓ Created project '{}' at {}", name, project_dir.display());
    
    Ok(())
}

fn show_config(config: &Config) -> Result<()> {
    println!("{}", toml::to_string_pretty(config)?);
    Ok(())
}

fn update_config(_settings: Vec<String>) -> Result<()> {
    println!("Configuration update not yet implemented");
    Ok(())
}

fn run_repl() -> Result<()> {
    use std::io::{self, BufRead, Write};
    
    println!("QB-COM Interactive Shell (REPL)");
    println!("Type 'exit' or 'quit' to exit, 'help' for commands");
    println!();
    
    let stdin = io::stdin();
    let mut line_num = 10;
    let mut program_lines: Vec<String> = Vec::new();
    
    print!("{} ", line_num);
    io::stdout().flush()?;
    
    for line in stdin.lock().lines() {
        let input = line?;
        
        if input.trim().eq_ignore_ascii_case("exit") || 
           input.trim().eq_ignore_ascii_case("quit") {
            break;
        }
        
        if input.trim().eq_ignore_ascii_case("help") {
            println!("Commands:");
            println!("  run    - Run the current program");
            println!("  clear  - Clear the current program");
            println!("  list   - List the current program");
            println!("  exit   - Exit the REPL");
            println!();
            print!("{} ", line_num);
            io::stdout().flush()?;
            continue;
        }
        
        if input.trim().eq_ignore_ascii_case("clear") {
            program_lines.clear();
            line_num = 10;
            println!("Program cleared.");
            print!("{} ", line_num);
            io::stdout().flush()?;
            continue;
        }
        
        if input.trim().eq_ignore_ascii_case("list") {
            if program_lines.is_empty() {
                println!("No program loaded.");
            } else {
                for (i, line) in program_lines.iter().enumerate() {
                    println!("{} {}", (i + 1) * 10, line);
                }
            }
            print!("{} ", line_num);
            io::stdout().flush()?;
            continue;
        }
        
        if input.trim().eq_ignore_ascii_case("run") {
            if program_lines.is_empty() {
                println!("No program to run.");
            } else {
                let source = program_lines.join("\n");
                match tokenize(&source) {
                    Ok(tokens) => {
                        match parse(tokens) {
                            Ok(ast) => {
                                match analyze(&ast) {
                                    Ok(_) => {
                                        match compile(&ast) {
                                            Ok(bytecode) => {
                                                if let Err(e) = run(&bytecode) {
                                                    eprintln!("Runtime error: {:?}", e);
                                                }
                                            }
                                            Err(e) => eprintln!("Compile error: {:?}", e),
                                        }
                                    }
                                    Err(e) => eprintln!("Analysis error: {:?}", e),
                                }
                            }
                            Err(e) => eprintln!("Parse error: {:?}", e),
                        }
                    }
                    Err(e) => eprintln!("Tokenize error: {:?}", e),
                }
            }
            print!("{} ", line_num);
            io::stdout().flush()?;
            continue;
        }
        
        if !input.trim().is_empty() {
            program_lines.push(input);
            line_num += 10;
        }
        
        print!("{} ", line_num);
        io::stdout().flush()?;
    }
    
    println!("\nGoodbye!");
    Ok(())
}
