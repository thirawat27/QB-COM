use thiserror::Error;

/// Error codes following QBasic style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QErrorCode {
    // File errors (50-75)
    FileNotFound = 53,
    DeviceIOError = 57,
    FileAlreadyExists = 58,
    BadFileName = 64,
    DiskFull = 61,
    InputPastEndOfFile = 62,
    BadFileMode = 54,
    FileAlreadyOpen = 55,
    BadRecordLength = 59,
    DiskNotReady = 71,
    RenameAcrossDisks = 74,
    PathFileAccessError = 75,

    // Device errors (25-26, 68-72)
    DeviceFault = 25,
    FatalError = 26,
    DeviceUnavailable = 68,
    CommunicationBufferOverflow = 69,
    DiskMediaError = 72,

    // Syntax and compile errors (1-9, 12-14, 18-20, 28-29, 33-38)
    NextWithoutFor = 1,
    SyntaxError = 2,
    ReturnWithoutGosub = 3,
    OutOfData = 4,
    IllegalFunctionCall = 5,
    Overflow = 6,
    OutOfMemory = 7,
    LabelNotDefined = 8,
    SubscriptOutOfRange = 9,
    DuplicateDefinition = 10,
    DivisionByZero = 11,
    TypeMismatch = 13,
    OutOfStringSpace = 14,
    StringFormulaTooComplex = 16,
    CannotContinue = 17,
    FunctionNotDefined = 18,
    NoResume = 19,
    ResumeWithoutError = 20,
    UnprintableError = 21,
    MissingOperand = 22,
    LineBufferOverflow = 23,
    AlreadyInContext = 28,
    FieldOverflow = 50,
    InternalError = 51,
    BadFileNumber = 52,
    UndefinedLineNumber = 90, // Different from LabelNotDefined
    BadRecordNumber = 63,
    Null = 94,

    // Feature errors (73-74)
    AdvancedFeatureUnavailable = 73,

    // System errors (100+)
    FeatureNotYetImplemented = 100,
    UnknownError = 255,
}

impl std::fmt::Display for QErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl QErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            QErrorCode::NextWithoutFor => "NEXT without FOR",
            QErrorCode::SyntaxError => "Syntax error",
            QErrorCode::ReturnWithoutGosub => "RETURN without GOSUB",
            QErrorCode::OutOfData => "Out of DATA",
            QErrorCode::IllegalFunctionCall => "Illegal function call",
            QErrorCode::Overflow => "Overflow",
            QErrorCode::OutOfMemory => "Out of memory",
            QErrorCode::LabelNotDefined => "Label not defined",
            QErrorCode::SubscriptOutOfRange => "Subscript out of range",
            QErrorCode::DuplicateDefinition => "Duplicate definition",
            QErrorCode::DivisionByZero => "Division by zero",
            QErrorCode::TypeMismatch => "Type mismatch",
            QErrorCode::OutOfStringSpace => "Out of string space",
            QErrorCode::StringFormulaTooComplex => "String formula too complex",
            QErrorCode::CannotContinue => "Cannot continue",
            QErrorCode::FunctionNotDefined => "Function not defined",
            QErrorCode::NoResume => "No RESUME",
            QErrorCode::ResumeWithoutError => "RESUME without error",
            QErrorCode::UnprintableError => "Unprintable error",
            QErrorCode::MissingOperand => "Missing operand",
            QErrorCode::LineBufferOverflow => "Line buffer overflow",
            QErrorCode::DeviceFault => "Device Fault",
            QErrorCode::FatalError => "Fatal error",
            QErrorCode::AlreadyInContext => "WHILE without WEND",
            QErrorCode::FieldOverflow => "FIELD overflow",
            QErrorCode::InternalError => "Internal error",
            QErrorCode::BadFileNumber => "Bad file number",
            QErrorCode::FileNotFound => "File not found",
            QErrorCode::DeviceUnavailable => "Device Unavailable",
            QErrorCode::CommunicationBufferOverflow => "Communication buffer overflow",
            QErrorCode::DeviceIOError => "Device I/O error",
            QErrorCode::FileAlreadyExists => "File already exists",
            QErrorCode::BadRecordLength => "Bad record length",
            QErrorCode::DiskFull => "Disk full",
            QErrorCode::InputPastEndOfFile => "Input past end of file",
            QErrorCode::BadRecordNumber => "Bad record number",
            QErrorCode::BadFileName => "Bad file name",
            QErrorCode::DiskNotReady => "Disk not ready",
            QErrorCode::DiskMediaError => "Disk media error",
            QErrorCode::AdvancedFeatureUnavailable => "Advanced feature unavailable",
            QErrorCode::PathFileAccessError => "Path/File access error",
            QErrorCode::RenameAcrossDisks => "Rename across disks",
            QErrorCode::BadFileMode => "Bad file mode",
            QErrorCode::FileAlreadyOpen => "File already open",
            QErrorCode::UndefinedLineNumber => "Undefined line number",
            QErrorCode::Null => "Null",
            QErrorCode::FeatureNotYetImplemented => "Feature not yet implemented",
            QErrorCode::UnknownError => "Unknown error",
        }
    }

    pub fn code(&self) -> i32 {
        *self as i32
    }
}

#[derive(Error, Debug, Clone)]
pub enum QError {
    #[error("Error {code}: {message} at line {line}, column {column}")]
    Runtime {
        code: QErrorCode,
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Compile Error: {message} at line {line}, column {column}")]
    Compile {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("IO Error: {0}")]
    Io(String),
    
    #[error("System Error: {0}")]
    System(String),
}

impl From<std::io::Error> for QError {
    fn from(e: std::io::Error) -> Self {
        QError::Io(e.to_string())
    }
}

impl QError {
    pub fn runtime(code: QErrorCode, line: usize, column: usize) -> Self {
        let message = code.as_str().to_string();
        QError::Runtime { code, message, line, column }
    }

    pub fn runtime_with_msg(code: QErrorCode, message: impl Into<String>, line: usize, column: usize) -> Self {
        QError::Runtime { code, message: message.into(), line, column }
    }

    pub fn compile(message: impl Into<String>, line: usize, column: usize) -> Self {
        QError::Compile { message: message.into(), line, column }
    }

    pub fn io(message: impl Into<String>) -> Self {
        QError::Io(message.into())
    }

    pub fn system(message: impl Into<String>) -> Self {
        QError::System(message.into())
    }
}

pub type QResult<T> = Result<T, QError>;
