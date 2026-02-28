//! QB-COM: Core Types Library
//! 
//! This crate provides the fundamental data types, memory emulation,
//! and error handling for the QBasic compiler.

pub mod data_types;
pub mod errors;
pub mod memory_map;

// Re-export commonly used items
pub use data_types::{
    ArrayBounds, CompareOp, ParamType, QType, TypeSuffix, UserTypeDef, VariableId, VariableRef,
};
pub use errors::{QError, QErrorCode, QResult};
pub use memory_map::{create_shared_memory, segments, DosMemory, SharedMemory};
