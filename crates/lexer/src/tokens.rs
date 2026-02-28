/// Token types for QBasic lexical analysis
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i32),           // Integer literal
    Long(i64),              // Long integer literal
    Single(f32),            // Single precision float
    Double(f64),            // Double precision float
    String(String),         // String literal
    
    // Identifiers
    Identifier(String),     // Variable/function name
    Label(String),          // Line label (ends with :)
    LineNumber(u32),        // Numeric line number
    
    // Keywords
    // Statements
    Rem,                    // Remark (comment)
    Let,                    // Variable assignment
    Const,                  // Constant declaration
    Dim,                    // Variable declaration
    Redim,                  // Redimension array
    Shared,                 // Shared variable
    Common,                 // Common variable
    Static,                 // Static variable
    DefInt,                 // Define default integer
    DefLng,                 // Define default long
    DefSng,                 // Define default single
    DefDbl,                 // Define default double
    DefStr,                 // Define default string
    Type,                   // User-defined type
    EndType,                // End of type definition
    
    // Control flow
    If,                     // If statement
    Then,                   // Then clause
    Else,                   // Else clause
    ElseIf,                 // Else if
    EndIf,                  // End if
    Select,                 // Select case
    Case,                   // Case statement
    CaseIs,                 // Case is
    CaseElse,               // Case else
    EndSelect,              // End select
    For,                    // For loop
    To,                     // To keyword
    Step,                   // Step keyword
    Next,                   // Next statement
    While,                  // While loop
    Wend,                   // End while
    Do,                     // Do loop
    Loop,                   // Loop statement
    Until,                  // Until condition
    GoTo,                   // Goto statement
    GoSub,                  // Gosub statement
    Return,                 // Return statement
    On,                     // On goto/gosub
    
    // Procedures
    Sub,                    // Subroutine
    EndSub,                 // End subroutine
    Function,               // Function
    EndFunction,            // End function
    Declare,                // Declare statement
    Call,                   // Call statement
    Exit,                   // Exit statement
    
    // I/O
    Print,                  // Print statement
    Input,                  // Input statement
    LineInput,              // Line input statement
    Write,                  // Write statement
    Open,                   // Open file
    Close,                  // Close file
    Output,                 // OPEN FOR OUTPUT
    Append,                 // OPEN FOR APPEND
    Random,                 // OPEN FOR RANDOM
    Binary,                 // OPEN FOR BINARY
    Get,                    // Get record
    Put,                    // Put record
    Seek,                   // Seek position
    Lock,                   // Lock file
    Unlock,                 // Unlock file
    InputHash,              // Input #
    PrintHash,              // Print #
    WriteHash,              // Write #
    
    // Graphics
    Screen,                 // Set screen mode
    PSet,                   // Set pixel
    PReset,                 // Reset pixel
    Line,                   // Draw line
    Circle,                 // Draw circle
    Draw,                   // Draw string
    Paint,                  // Flood fill
    View,                   // Set viewport
    Window,                 // Set window
    Palette,                // Set palette
    Color,                  // Set color
    Cls,                    // Clear screen
    Locate,                 // Position cursor
    Width,                  // Set width
    
    // Sound
    Beep,                   // Beep
    Sound,                  // Sound
    Play,                   // Play music
    
    // Memory & System
    Poke,                   // Write to memory
    Peek,                   // Read from memory
    InP,                    // Input from port
    Out,                    // Output to port
    Wait,                   // Wait for port
    DefSeg,                 // Define segment
    VarPtr,                 // Get variable pointer
    VarSeg,                 // Get variable segment
    
    // Error handling
    OnError,                // On error
    Resume,                 // Resume
    ResumeNext,             // Resume next
    Error,                  // Error statement
    Err,                    // Error number
    ERL,                    // Error line
    
    // Data
    Data,                   // Data statement
    Read,                   // Read data
    Restore,                // Restore data pointer
    
    // Environment
    Environ,                // Environment variable
    Shell,                  // Execute shell command
    System,                 // Exit to system
    End,                    // End program
    Stop,                   // Stop execution
    
    // Operators
    Plus,                   // +
    Minus,                  // -
    Multiply,               // *
    Divide,                 // /
    IntDivide,              // \
    Modulo,                 // MOD
    Power,                  // ^
    Concat,                 // +
    
    // Comparison operators
    Equal,                  // =
    NotEqual,               // <> or ><
    Less,                   // <
    LessEqual,              // <= or =<
    Greater,                // >
    GreaterEqual,           // >= or =>
    
    // Logical operators
    And,                    // AND
    Or,                     // OR
    Xor,                    // XOR
    Not,                    // NOT
    Imp,                    // IMP
    Eqv,                    // EQV
    
    // Bitwise operators (same as logical for integers)
    
    // Other keywords
    As,                     // As keyword
    Is,                     // Is keyword
    Len,                    // Length
    Using,                  // Using format
    
    // Built-in functions (math)
    Abs, Atn, Cos, Exp, Fix, Int, Log, Randomize, Rnd, Sgn, Sin, Sqr, Tan,
    
    // Built-in functions (string)
    Asc, Chr, Cvi, Cvs, Cvd, InStr, Left, LenFunc, LSet, Mid, 
    MkD, MkI, MkL, MkS, Oct, Right, RSet, Space, Str, StringFunc,
    Trim, LTrim, RTrim, UCase, LCase, InKey, 
    
    // Built-in functions (type conversion)
    CBool, CByte, CInt, CLng, CSng, CDbl, CStr, CDate, CCur, CVar, CVErr, Val,
    
    // Built-in functions (date/time)
    Date, DateFunc, Time, TimeFunc, Timer, 
    
    // Built-in functions (file)
    Eof, Lof, Loc, SeekFunc, FreeFile,
    
    // Built-in functions (misc)
    Command, Dir, FileAttr, FileDateTime, FileLen, 
    GetAttr, InputFunc, IOStat, LBound, UBound,
    Saddle, SAdd,
    
    // Type suffixes
    IntegerSuffix,          // %
    LongSuffix,             // &
    SingleSuffix,           // !
    DoubleSuffix,           // #
    StringSuffix,           // $
    
    // Delimiters
    LParen,                 // (
    RParen,                 // )
    LBracket,               // [
    RBracket,               // ]
    LBrace,                 // {
    RBrace,                 // }
    Comma,                  // ,
    Semicolon,              // ;
    Colon,                  // :
    Period,                 // .
    Hash,                   // #
    Apostrophe,             // '
    
    // Special
    NewLine,                // Line break
    EOF,                    // End of file
    Underscore,             // _ (line continuation)
    
    // Type keywords
    IntegerType,            // INTEGER
    LongType,               // LONG
    SingleType,             // SINGLE
    DoubleType,             // DOUBLE
    StringType,             // STRING
    VariantType,            // VARIANT
    AnyType,                // ANY
    
    // QB64 Extended types
    Integer64Type,          // _INTEGER64
    UnsignedIntegerType,    // _UNSIGNED INTEGER
    UnsignedLongType,       // _UNSIGNED LONG
    UnsignedInteger64Type,  // _UNSIGNED _INTEGER64
    FloatType,              // _FLOAT
    
    // QB64 Metacommands
    MetaDynamic,            // $DYNAMIC
    MetaStatic,             // $STATIC
    MetaInclude,            // $INCLUDE
    MetaIf,                 // $IF
    MetaElse,               // $ELSE
    MetaEndIf,              // $END IF
    MetaResize,             // $RESIZE
    MetaConsole,            // $CONSOLE
    MetaScreenShow,         // $SCREENHIDE/_SCREENSHOW
    
    // QB64 Graphics commands
    NewImage,               // _NEWIMAGE
    LoadImage,              // _LOADIMAGE
    PutImage,               // _PUTIMAGE
    GetImage,               // _GETIMAGE
    ScreenImage,            // _SCREENIMAGE
    RGB,                    // _RGB
    RGBA,                   // _RGBA
    Red,                    // _RED
    Green,                  // _GREEN
    Blue,                   // _BLUE
    Alpha,                  // _ALPHA
    
    // QB64 Sound commands
    SndOpen,                // _SNDOPEN
    SndPlay,                // _SNDPLAY
    SndLoop,                // _SNDLOOP
    SndClose,               // _SNDCLOSE
    
    // QB64 Input/Events
    MouseInput,             // _MOUSEINPUT
    MouseX,                 // _MOUSEX
    MouseY,                 // _MOUSEY
    MouseButton,            // _MOUSEBUTTON
    MouseWheel,             // _MOUSEWHEEL
    KeyHit,                 // _KEYHIT
    KeyClear,               // _KEYCLEAR
    // QB64 Screen/Window
    Resize,                 // _RESIZE
    QB64Width,              // _WIDTH
    Height,                 // _HEIGHT
    Font,                   // _FONT
    PrintString,            // _PRINTSTRING
    
    // QB64 Math/Other
    Define,                 // _DEFINE
    Preserve,               // _PRESERVE
    FreeImage,              // _FREEIMAGE
    CopyImage,              // _COPYIMAGE
    Limit,                  // _LIMIT
    Display,                // _DISPLAY
    AutoDisplay,            // _AUTODISPLAY
    FullScreen,             // _FULLSCREEN
    AllowFullScreen,        // _ALLOWFULLSCREEN
    Console,                // _CONSOLE
    ScreenShow,             // _SCREENSHOW
    ScreenHide,             // _SCREENHIDE
}

impl Token {
    /// Check if token is a type suffix
    pub fn is_type_suffix(&self) -> bool {
        matches!(self, 
            Token::IntegerSuffix | 
            Token::LongSuffix | 
            Token::SingleSuffix | 
            Token::DoubleSuffix | 
            Token::StringSuffix
        )
    }

    /// Check if token is a binary operator
    pub fn is_binary_op(&self) -> bool {
        matches!(self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::IntDivide | Token::Modulo | Token::Power |
            Token::Equal | Token::NotEqual | Token::Less | Token::LessEqual |
            Token::Greater | Token::GreaterEqual |
            Token::And | Token::Or | Token::Xor | Token::Imp | Token::Eqv
        )
    }

    /// Check if token is a unary operator
    pub fn is_unary_op(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Not)
    }

    /// Check if token is a statement keyword
    pub fn is_statement(&self) -> bool {
        matches!(self,
            Token::Rem | Token::Let | Token::Const | Token::Dim | Token::Redim |
            Token::Shared | Token::Common | Token::Static | Token::Type |
            Token::If | Token::Select | Token::For | Token::While | Token::Do |
            Token::GoTo | Token::GoSub | Token::On | Token::Sub | Token::Function |
            Token::Declare | Token::Call | Token::Exit | Token::Print | Token::Input |
            Token::LineInput | Token::Write | Token::Open | Token::Close |
            Token::Get | Token::Put | Token::Seek | Token::Lock | Token::Unlock |
            Token::Screen | Token::PSet | Token::PReset | Token::Line | Token::Circle |
            Token::Draw | Token::Paint | Token::View | Token::Window | Token::Palette |
            Token::Color | Token::Cls | Token::Locate | Token::Width |
            Token::Beep | Token::Sound | Token::Play | Token::Poke | Token::Wait |
            Token::DefSeg | Token::Data | Token::Read | Token::Restore |
            Token::Environ | Token::Shell | Token::System | Token::End | Token::Stop |
            Token::Resume | Token::Error
        )
    }

    /// Get keyword precedence for parsing
    pub fn precedence(&self) -> i32 {
        match self {
            Token::Or => 1,
            Token::Xor => 2,
            Token::And => 3,
            Token::Eqv => 4,
            Token::Imp => 5,
            Token::Equal | Token::NotEqual => 6,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => 7,
            Token::Plus | Token::Minus | Token::Concat => 8,
            Token::Modulo => 9,
            Token::IntDivide => 10,
            Token::Multiply | Token::Divide => 11,
            Token::Power => 12,
            _ => 0,
        }
    }

    /// Get the string name if token is a builtin function
    pub fn as_builtin_function_name(&self) -> Option<&'static str> {
        match self {
            Token::Abs => Some("ABS"),
            Token::Atn => Some("ATN"),
            Token::Cos => Some("COS"),
            Token::Exp => Some("EXP"),
            Token::Fix => Some("FIX"),
            Token::Int => Some("INT"),
            Token::Log => Some("LOG"),
            Token::Rnd => Some("RND"),
            Token::Sgn => Some("SGN"),
            Token::Sin => Some("SIN"),
            Token::Sqr => Some("SQR"),
            Token::Tan => Some("TAN"),
            Token::Asc => Some("ASC"),
            Token::Chr => Some("CHR$"),
            Token::Left => Some("LEFT$"),
            Token::Len | Token::LenFunc => Some("LEN"),
            Token::Mid => Some("MID$"),
            Token::Right => Some("RIGHT$"),
            Token::Str => Some("STR$"),
            Token::Val => Some("VAL"),
            Token::CInt => Some("CINT"),
            Token::CLng => Some("CLNG"),
            Token::CSng => Some("CSNG"),
            Token::CDbl => Some("CDBL"),
            Token::CStr => Some("CSTR"),
            Token::InStr => Some("INSTR"),
            Token::LCase => Some("LCASE$"),
            Token::UCase => Some("UCASE$"),
            Token::Trim => Some("TRIM$"),
            Token::LTrim => Some("LTRIM$"),
            Token::RTrim => Some("RTRIM$"),
            Token::Space => Some("SPACE$"),
            Token::StringFunc => Some("STRING$"),
            Token::Timer => Some("TIMER"),
            // Can be expanded as needed
            _ => None,
        }
    }
}

/// Convert string to keyword token
pub fn string_to_keyword(s: &str) -> Option<Token> {
    let upper = s.to_uppercase();
    Some(match upper.as_str() {
        // Comments
        "REM" => Token::Rem,
        
        // Declaration
        "LET" => Token::Let,
        "CONST" => Token::Const,
        "DIM" => Token::Dim,
        "REDIM" => Token::Redim,
        "SHARED" => Token::Shared,
        "COMMON" => Token::Common,
        "STATIC" => Token::Static,
        "DEFINT" => Token::DefInt,
        "DEFLNG" => Token::DefLng,
        "DEFSNG" => Token::DefSng,
        "DEFDBL" => Token::DefDbl,
        "DEFSTR" => Token::DefStr,
        
        // Control flow
        "IF" => Token::If,
        "THEN" => Token::Then,
        "ELSE" => Token::Else,
        "ELSEIF" => Token::ElseIf,
        "ENDIF" => Token::EndIf,
        "END" => Token::End,
        "SELECT" => Token::Select,
        "CASE" => Token::Case,
        "CASEIS" => Token::CaseIs,
        "CASEELSE" => Token::CaseElse,
        "ENDSELECT" => Token::EndSelect,
        "FOR" => Token::For,
        "TO" => Token::To,
        "STEP" => Token::Step,
        "NEXT" => Token::Next,
        "WHILE" => Token::While,
        "WEND" => Token::Wend,
        "DO" => Token::Do,
        "LOOP" => Token::Loop,
        "UNTIL" => Token::Until,
        "GOTO" => Token::GoTo,
        "GOSUB" => Token::GoSub,
        "RETURN" => Token::Return,
        "ON" => Token::On,
        
        // Procedures
        "SUB" => Token::Sub,
        "FUNCTION" => Token::Function,
        "DECLARE" => Token::Declare,
        "CALL" => Token::Call,
        "EXIT" => Token::Exit,
        
        // I/O
        "PRINT" => Token::Print,
        "INPUT" => Token::Input,
        "OUTPUT" => Token::Output,
        "APPEND" => Token::Append,
        "RANDOM" => Token::Random,
        "BINARY" => Token::Binary,

        "WRITE" => Token::Write,
        "OPEN" => Token::Open,
        "CLOSE" => Token::Close,
        "GET" => Token::Get,
        "PUT" => Token::Put,
        "SEEK" => Token::Seek,
        "LOCK" => Token::Lock,
        "UNLOCK" => Token::Unlock,
        
        // Graphics
        "SCREEN" => Token::Screen,
        "PSET" => Token::PSet,
        "PRESET" => Token::PReset,
        "LINE" => Token::Line,
        "CIRCLE" => Token::Circle,
        "DRAW" => Token::Draw,
        "PAINT" => Token::Paint,
        "VIEW" => Token::View,
        "WINDOW" => Token::Window,
        "PALETTE" => Token::Palette,
        "COLOR" => Token::Color,
        "CLS" => Token::Cls,
        "LOCATE" => Token::Locate,
        "WIDTH" => Token::Width,
        
        // Sound
        "BEEP" => Token::Beep,
        "SOUND" => Token::Sound,
        "PLAY" => Token::Play,
        
        // Memory & System
        "POKE" => Token::Poke,
        "PEEK" => Token::Peek,
        "INP" => Token::InP,
        "OUT" => Token::Out,
        "WAIT" => Token::Wait,
        "DEFSEG" => Token::DefSeg,
        "VARPTR" => Token::VarPtr,
        "VARSEG" => Token::VarSeg,
        
        // Error handling
        "ERROR" => Token::Error,
        "RESUME" => Token::Resume,
        "ERR" => Token::Err,
        "ERL" => Token::ERL,
        "STOP" => Token::Stop,
        
        // Data
        "DATA" => Token::Data,
        "READ" => Token::Read,
        "RESTORE" => Token::Restore,
        
        // Environment
        "ENVIRON" => Token::Environ,
        "SHELL" => Token::Shell,
        "SYSTEM" => Token::System,
        
        // Types
        "AS" => Token::As,
        "IS" => Token::Is,
        "TYPE" => Token::Type,
        "LEN" => Token::Len,
        "USING" => Token::Using,
        
        // Type keywords
        "INTEGER" => Token::IntegerType,
        "LONG" => Token::LongType,
        "SINGLE" => Token::SingleType,
        "DOUBLE" => Token::DoubleType,
        "STRING" => Token::StringType,
        "VARIANT" => Token::VariantType,
        "ANY" => Token::AnyType,
        
        // Logical operators
        "AND" => Token::And,
        "OR" => Token::Or,
        "XOR" => Token::Xor,
        "NOT" => Token::Not,
        "IMP" => Token::Imp,
        "EQV" => Token::Eqv,
        "MOD" => Token::Modulo,
        
        // Math functions
        "ABS" => Token::Abs,
        "ATN" => Token::Atn,
        "COS" => Token::Cos,
        "EXP" => Token::Exp,
        "FIX" => Token::Fix,
        "INT" => Token::Int,
        "LOG" => Token::Log,
        "RANDOMIZE" => Token::Randomize,
        "RND" => Token::Rnd,
        "SGN" => Token::Sgn,
        "SIN" => Token::Sin,
        "SQR" => Token::Sqr,
        "TAN" => Token::Tan,
        
        // String functions
        "ASC" => Token::Asc,
        "CHR$" => Token::Chr,
        "CVI" => Token::Cvi,
        "CVS" => Token::Cvs,
        "CVD" => Token::Cvd,
        "INSTR" => Token::InStr,
        "LEFT$" => Token::Left,
        "LSET" => Token::LSet,
        "MID$" => Token::Mid,
        "MKD$" => Token::MkD,
        "MKI$" => Token::MkI,
        "MKL$" => Token::MkL,
        "MKS$" => Token::MkS,
        "OCT$" => Token::Oct,
        "RIGHT$" => Token::Right,
        "RSET" => Token::RSet,
        "SPACE$" => Token::Space,
        "STR$" => Token::Str,
        "STRING$" => Token::StringFunc,
        "LCASE$" => Token::LCase,
        "UCASE$" => Token::UCase,
        "LTRIM$" => Token::LTrim,
        "RTRIM$" => Token::RTrim,
        "TRIM$" => Token::Trim,
        "INKEY$" => Token::InKey,
        
        // Type conversion
        "CBOOL" => Token::CBool,
        "CBYTE" => Token::CByte,
        "CINT" => Token::CInt,
        "CLNG" => Token::CLng,
        "CSNG" => Token::CSng,
        "CDBL" => Token::CDbl,
        "CSTR" => Token::CStr,
        "CDATE" => Token::CDate,
        "CCUR" => Token::CCur,
        "CVAR" => Token::CVar,
        "CVERR" => Token::CVErr,
        "VAL" => Token::Val,
        
        // Date/Time
        "DATE$" => Token::Date,
        "TIME$" => Token::Time,
        "TIMER" => Token::Timer,
        
        // File functions
        "EOF" => Token::Eof,
        "LOF" => Token::Lof,
        "LOC" => Token::Loc,
        "FREEFILE" => Token::FreeFile,
        
        // Other functions
        "COMMAND$" => Token::Command,
        "DIR$" => Token::Dir,
        "INPUT$" => Token::InputFunc,
        "LBOUND" => Token::LBound,
        "UBOUND" => Token::UBound,
        "SADD" => Token::SAdd,
        "SADDLE" => Token::Saddle,
        
        // QB64 Extended types
        "_INTEGER64" => Token::Integer64Type,
        "_UNSIGNED" => Token::UnsignedIntegerType,
        "_FLOAT" => Token::FloatType,
        
        // QB64 Metacommands
        "$DYNAMIC" => Token::MetaDynamic,
        "$STATIC" => Token::MetaStatic,
        "$INCLUDE" => Token::MetaInclude,
        "$IF" => Token::MetaIf,
        "$ELSE" => Token::MetaElse,
        "$END" => Token::MetaEndIf,
        "$RESIZE" => Token::MetaResize,
        "$CONSOLE" => Token::MetaConsole,
        "$SCREENSHOW" => Token::MetaScreenShow,
        "$SCREENHIDE" => Token::ScreenHide,
        
        // QB64 Graphics
        "_NEWIMAGE" => Token::NewImage,
        "_LOADIMAGE" => Token::LoadImage,
        "_PUTIMAGE" => Token::PutImage,
        "_GETIMAGE" => Token::GetImage,
        "_SCREENIMAGE" => Token::ScreenImage,
        "_COPYIMAGE" => Token::CopyImage,
        "_FREEIMAGE" => Token::FreeImage,
        "_RGB" => Token::RGB,
        "_RGBA" => Token::RGBA,
        "_RED" => Token::Red,
        "_GREEN" => Token::Green,
        "_BLUE" => Token::Blue,
        "_ALPHA" => Token::Alpha,
        
        // QB64 Sound
        "_SNDOPEN" => Token::SndOpen,
        "_SNDPLAY" => Token::SndPlay,
        "_SNDLOOP" => Token::SndLoop,
        "_SNDCLOSE" => Token::SndClose,
        
        // QB64 Input/Events
        "_MOUSEINPUT" => Token::MouseInput,
        "_MOUSEX" => Token::MouseX,
        "_MOUSEY" => Token::MouseY,
        "_MOUSEBUTTON" => Token::MouseButton,
        "_MOUSEWHEEL" => Token::MouseWheel,
        "_KEYHIT" => Token::KeyHit,
        "_KEYCLEAR" => Token::KeyClear,
        "_INKEY$" => Token::InKey,
        
        // QB64 Screen/Window
        "_RESIZE" => Token::Resize,
        "_WIDTH" => Token::Width,
        "_HEIGHT" => Token::Height,
        "_FONT" => Token::Font,
        "_PRINTSTRING" => Token::PrintString,
        "_FULLSCREEN" => Token::FullScreen,
        "_ALLOWFULLSCREEN" => Token::AllowFullScreen,
        "_DISPLAY" => Token::Display,
        "_AUTODISPLAY" => Token::AutoDisplay,
        "_LIMIT" => Token::Limit,
        "_CONSOLE" => Token::Console,
        
        // QB64 Other
        "_DEFINE" => Token::Define,
        "_PRESERVE" => Token::Preserve,
        
        _ => return None,
    })
}

/// Token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl TokenInfo {
    pub fn new(token: Token, line: usize, column: usize, length: usize) -> Self {
        Self { token, line, column, length }
    }
}
