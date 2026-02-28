use qb_core::data_types::QType;
use serde::{Deserialize, Serialize};

/// Bytecode instructions for the QBasic VM
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpCode {
    // Stack operations
    Push(QType),           // Push literal value
    Pop,                   // Pop value from stack
    Dup,                   // Duplicate top of stack
    Swap,                  // Swap top two stack items
    
    // Variable operations
    LoadVar(String),       // Load variable onto stack
    StoreVar(String),      // Store top of stack to variable
    LoadArray(String, usize), // Load array element
    StoreArray(String, usize), // Store to array element
    LoadField(String, String), // Load field from record (var, field)
    StoreField(String, String), // Store to field in record (var, field)
    DimArray(String, Vec<(i32, i32)>, String), // Create array with shape [(lo, hi), ...] and type
    
    // Arithmetic operations
    Add,                   // Pop two values, push sum
    Sub,                   // Pop two values, push difference
    Mul,                   // Pop two values, push product
    Div,                   // Pop two values, push quotient (float)
    IntDiv,                // Integer division
    Mod,                   // Modulo
    Pow,                   // Power
    Neg,                   // Negate
    
    // Bitwise operations
    BitNot,                // Bitwise NOT
    BitAnd,                // Bitwise AND
    BitOr,                 // Bitwise OR
    BitXor,                // Bitwise XOR
    BitImp,                // Bitwise IMP
    BitEqv,                // Bitwise EQV
    
    // Comparison operations
    Eq,                    // Equal
    Ne,                    // Not equal
    Lt,                    // Less than
    Le,                    // Less or equal
    Gt,                    // Greater than
    Ge,                    // Greater or equal
    
    // Logical operations
    LogNot,                // Logical NOT
    LogAnd,                // Logical AND
    LogOr,                 // Logical OR
    
    // Control flow
    Jump(u32),             // Unconditional jump
    JumpIfTrue(u32),       // Jump if top of stack is true
    JumpIfFalse(u32),      // Jump if top of stack is false
    Call(u32),             // Call subroutine
    Return,                // Return from subroutine
    
    // I/O operations
    Print(bool),           // Print with newline (true) or not
    PrintComma,            // Print tab
    PrintSemicolon,        // Print nothing (continue on same line)
    PrintHash(u8),         // Print to file
    Input(String),         // Input with prompt
    LineInput(String),     // Line input with prompt
    InputHash(u8),         // Input from file
    Open(String, String, u8), // Open file (filename, mode, fileno)
    Close(u8),             // Close file
    WriteHash(u8),         // Write to file
    
    // Graphics operations
    Screen(u8),            // Set screen mode
    PSet,                  // Set pixel
    PReset,                // Reset pixel
    Line,                  // Draw line
    Circle,                // Draw circle
    Cls,                   // Clear screen
    Color,                 // Set color
    Locate,                // Position cursor
    
    // QB64 Graphics extensions
    RGB(u8, u8, u8),       // Create RGB color
    RGBA(u8, u8, u8, u8),  // Create RGBA color
    NewImage(i32, i32, u8), // Create new image buffer
    LoadImage(String),     // Load image from file
    PutImage,              // Draw image to screen
    
    // QB64 Sound extensions
    SndOpen(String),       // Open sound file
    SndClose(i32),         // Close sound handle
    SndPlay(i32),          // Play sound
    SndStop(i32),          // Stop sound
    SndLoop(i32),          // Loop sound
    SndVolume(i32, f32),   // Set sound volume
    
    // Sound operations
    Beep,                  // Beep
    Sound,                 // Sound frequency, duration
    Play,                  // Play music string
    
    // Memory operations
    Peek,                  // Peek from memory
    Poke,                  // Poke to memory
    DefSeg(u16),           // Define segment
    
    // String operations
    Concat,                // String concatenation
    Left,                  // Left$(string, count)
    Right,                 // Right$(string, count)
    Mid,                   // Mid$(string, start, length)
    Len,                   // Len(string)
    Asc,                   // Asc(char)
    Chr,                   // Chr$(code)
    Str,                   // Str$(number)
    Val,                   // Val(string)
    UCase,                 // UCase$(string)
    LCase,                 // LCase$(string)
    // Type conversion
    CInt,                  // Convert to integer
    CLng,                  // Convert to long
    CSng,                  // Convert to single
    CDbl,                  // Convert to double
    CStr,                  // Convert to string
    
    // Math operations
    Abs,
    Atn,
    Cos,
    Exp,
    Fix,
    IntOp,                 // Int
    Log,
    Rnd,
    Sgn,
    Sin,
    Sqr,
    Tan,
    
    // Function/Subroutine
    PushRet(u32),          // Push return address
    PopRet,                // Pop return address
    EnterScope,            // Enter new scope
    ExitScope,             // Exit scope
    
    // Data operations
    Read,                  // Read from DATA
    Restore(u32),          // Restore DATA pointer
    
    // Program control
    End,                   // End program
    Stop,                  // Stop execution
    
    // Special
    Nop,                   // No operation
    Halt,                  // Halt execution
}

/// Compiled bytecode chunk
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ByteCode {
    pub instructions: Vec<OpCode>,
    pub constants: Vec<QType>,
    pub data_items: Vec<QType>, // DATA statements
}

impl ByteCode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, op: OpCode) -> usize {
        self.instructions.push(op);
        self.instructions.len() - 1
    }

    pub fn emit_at(&mut self, index: usize, op: OpCode) {
        self.instructions[index] = op;
    }

    pub fn add_constant(&mut self, value: QType) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn add_data(&mut self, value: QType) {
        self.data_items.push(value);
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
