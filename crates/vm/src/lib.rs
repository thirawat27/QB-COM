//! QB-VM: Virtual Machine for QBasic
//! 
//! Provides bytecode compiler and virtual machine for executing QBasic programs.

pub mod opcodes;
pub mod compiler;
pub mod runtime;

pub use opcodes::{ByteCode, OpCode};
pub use compiler::{ByteCodeCompiler, compile};
pub use runtime::{VirtualMachine, run};
