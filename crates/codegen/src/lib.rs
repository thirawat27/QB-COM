//! QB-Codegen: Native Code Generator for QBasic
//! 
//! Provides LLVM-based native code compilation for QBasic programs.
//! This is a placeholder for future LLVM backend implementation.

use qb_core::errors::{QError, QErrorCode, QResult};
use qb_parser::ast_nodes::Program;

/// Native code generator using LLVM
pub struct NativeCodeGenerator;

impl NativeCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Compile QBasic program to native executable
    pub fn compile(&self, _program: &Program, _output_path: &str) -> QResult<()> {
        // Placeholder - LLVM backend not yet implemented
        Err(QError::runtime(
            QErrorCode::AdvancedFeatureUnavailable,
            0,
            0,
        ))
    }
}

impl Default for NativeCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile a program to a native executable
pub fn compile_to_native(_program: &Program, _output_path: &str) -> QResult<()> {
    let generator = NativeCodeGenerator::new();
    generator.compile(_program, _output_path)
}
