//! QB-Semantic: Semantic Analyzer for QBasic
//! 
//! Provides semantic analysis and type checking for QBasic.

pub mod scope;
pub mod type_checker;

pub use scope::{Scope, SymbolTable};
pub use type_checker::{TypeChecker, analyze};
