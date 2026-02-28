use qb_core::data_types::{ArrayBounds, ParamType, VariableId};
use qb_lexer::tokens::Token;

/// The complete Abstract Syntax Tree for a QBasic program
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub line_numbers: std::collections::HashMap<u32, usize>, // Line number -> statement index
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            line_numbers: std::collections::HashMap::new(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

/// All possible QBasic statements
#[derive(Debug, Clone)]
pub enum Statement {
    // Comments
    Rem(String),
    
    // Declarations
    Dim {
        vars: Vec<DimItem>,
    },
    Const {
        name: VariableId,
        value: Expression,
    },
    DefType {
        type_char: char, // I, L, S, D, or $
        letter_range: (char, char),
    },
    TypeDef {
        name: String,
        fields: Vec<(String, TypeSpec)>,
    },
    
    // Control Flow
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_if_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
        is_single_line: bool,
    },
    Select {
        expr: Expression,
        cases: Vec<CaseClause>,
        case_else: Option<Vec<Statement>>,
    },
    For {
        var: VariableId,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoWhile {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoUntil {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoLoop {
        body: Vec<Statement>,
        condition: Option<Expression>,
        is_until: bool,
    },
    
    // Jumps
    Goto {
        label: String,
    },
    Gosub {
        label: String,
    },
    Return,
    OnGoto {
        expr: Expression,
        labels: Vec<String>,
    },
    OnGosub {
        expr: Expression,
        labels: Vec<String>,
    },
    
    // Procedures
    Sub {
        name: String,
        params: Vec<ParamType>,
        body: Vec<Statement>,
        is_static: bool,
    },
    Function {
        name: String,
        params: Vec<ParamType>,
        return_type: Option<TypeSpec>,
        body: Vec<Statement>,
        is_static: bool,
    },
    Declare {
        is_sub: bool,
        name: String,
        params: Vec<ParamType>,
    },
    Call {
        name: String,
        args: Vec<Argument>,
    },
    ExitSub,
    ExitFunction,
    ExitFor,
    ExitDo,
    
    // I/O
    Print {
        items: Vec<PrintItem>,
        is_question: bool, // PRINT vs ?
    },
    Input {
        prompt: Option<String>,
        vars: Vec<VariableId>,
    },
    PrintHash {
        fileno: Expression,
        items: Vec<PrintItem>,
    },
    InputHash {
        fileno: Expression,
        vars: Vec<VariableId>,
    },
    LineInput {
        prompt: Option<String>,
        var: VariableId,
    },
    Write {
        items: Vec<Expression>,
    },
    
    // File I/O
    Open {
        filename: Expression,
        mode: FileMode,
        fileno: Expression,
        reclen: Option<Expression>,
    },
    Close {
        fileno: Option<Expression>,
    },
    Get {
        fileno: Expression,
        record: Option<Expression>,
        var: VariableId,
    },
    Put {
        fileno: Expression,
        record: Option<Expression>,
        var: VariableId,
    },
    Seek {
        fileno: Expression,
        position: Expression,
    },
    PrintFile {
        fileno: Expression,
        items: Vec<PrintItem>,
    },
    InputFile {
        fileno: Expression,
        vars: Vec<VariableId>,
    },
    Lock {
        fileno: Expression,
        record: Option<(Expression, Option<Expression>)>,
    },
    Unlock {
        fileno: Expression,
        record: Option<(Expression, Option<Expression>)>,
    },
    
    // Graphics
    Screen {
        mode: Expression,
    },
    PSet {
        x: Expression,
        y: Expression,
        color: Option<Expression>,
    },
    PReset {
        x: Expression,
        y: Expression,
    },
    Line {
        x1: Expression,
        y1: Expression,
        x2: Expression,
        y2: Expression,
        color: Option<Expression>,
        style: Option<Expression>,
        is_box: bool,
        is_filled: bool,
    },
    Circle {
        x: Expression,
        y: Expression,
        radius: Expression,
        color: Option<Expression>,
        start: Option<Expression>,
        end: Option<Expression>,
        aspect: Option<Expression>,
    },
    Draw {
        command: Expression,
    },
    Paint {
        x: Expression,
        y: Expression,
        paint_color: Option<Expression>,
        border_color: Option<Expression>,
    },
    View {
        x1: Expression,
        y1: Expression,
        x2: Expression,
        y2: Expression,
        color: Option<Expression>,
        border: Option<Expression>,
    },
    Window {
        x1: Expression,
        y1: Expression,
        x2: Expression,
        y2: Expression,
        screen_coords: bool,
    },
    Palette {
        attribute: Option<Expression>,
        color: Option<Expression>,
    },
    Color {
        foreground: Option<Expression>,
        background: Option<Expression>,
        border: Option<Expression>,
    },
    Cls,
    Locate {
        row: Option<Expression>,
        col: Option<Expression>,
        cursor: Option<Expression>,
        start: Option<Expression>,
        stop: Option<Expression>,
    },
    Width {
        value: Expression,
    },
    
    // Sound
    Beep,
    Sound {
        frequency: Expression,
        duration: Expression,
    },
    Play {
        command: Expression,
    },
    
    // Memory
    Poke {
        address: Expression,
        value: Expression,
    },
    DefSeg {
        segment: Option<Expression>,
    },
    
    // Data
    Data {
        values: Vec<Expression>,
    },
    Read {
        vars: Vec<VariableId>,
    },
    Restore {
        label: Option<String>,
    },
    
    // Environment
    Environ {
        expr: Expression,
    },
    Shell {
        command: Option<Expression>,
    },
    System,
    
    // Error handling
    OnError {
        label: String,
    },
    Resume {
        next: bool,
        label: Option<String>,
    },
    Error {
        code: Expression,
    },
    
    // Program flow
    End,
    Stop,
    
    // Other
    Assignment {
        target: LValue,
        value: Expression,
    },
    Label {
        name: String,
    },
    LineNumber {
        number: u32,
    },
}

/// Dimensional item (for DIM statement)
#[derive(Debug, Clone)]
pub struct DimItem {
    pub name: VariableId,
    pub bounds: Option<Vec<ArrayBounds>>,
    pub type_spec: Option<TypeSpec>,
    pub shared: bool,
}

/// Type specification
#[derive(Debug, Clone)]
pub enum TypeSpec {
    Simple(String),           // INTEGER, LONG, SINGLE, DOUBLE, STRING
    FixedString(Expression),  // STRING * length
    UserDefined(String),      // User-defined type name
}

/// File access mode
#[derive(Debug, Clone, Copy)]
pub enum FileMode {
    Input,
    Output,
    Append,
    Random,
    Binary,
}

/// Print item (expression or separator)
#[derive(Debug, Clone)]
pub enum PrintItem {
    Expression(Expression),
    Semicolon,
    Comma,
}

/// Argument for procedure calls
#[derive(Debug, Clone)]
pub enum Argument {
    ByVal(Expression),
    ByRef(VariableId),
}

/// Case clause for SELECT statement
#[derive(Debug, Clone)]
pub struct CaseClause {
    pub conditions: Vec<CaseCondition>,
    pub body: Vec<Statement>,
}

/// Case condition
#[derive(Debug, Clone)]
pub enum CaseCondition {
    Expression(Expression),
    Range(Expression, Expression),
    Is(Token, Expression), // Comparison operator and expression
}

/// LValue (left-hand side of assignment)
#[derive(Debug, Clone)]
pub enum LValue {
    Variable(VariableId),
    ArrayElement(VariableId, Vec<Expression>),
    Field(Box<LValue>, String), // Record.field
}

/// All possible expressions
#[derive(Debug, Clone)]
pub enum Expression {
    // Literals
    Integer(i32),
    Long(i64),
    Single(f32),
    Double(f64),
    String(String),
    Empty,
    
    // Variables
    Variable(VariableId),
    ArrayAccess(VariableId, Vec<Expression>),
    FieldAccess(Box<Expression>, String),
    
    // Unary operations
    Negate(Box<Expression>),
    Not(Box<Expression>),
    
    // Binary operations
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    
    // Function calls
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    
    // Type conversion
    TypeConversion {
        target_type: String,
        expr: Box<Expression>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntDivide,
    Modulo,
    Power,
    Concat,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Xor,
    Imp,
    Eqv,
}

impl BinaryOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        Some(match token {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Subtract,
            Token::Multiply => BinaryOp::Multiply,
            Token::Divide => BinaryOp::Divide,
            Token::IntDivide => BinaryOp::IntDivide,
            Token::Modulo => BinaryOp::Modulo,
            Token::Power => BinaryOp::Power,
            Token::Equal => BinaryOp::Equal,
            Token::NotEqual => BinaryOp::NotEqual,
            Token::Less => BinaryOp::Less,
            Token::LessEqual => BinaryOp::LessEqual,
            Token::Greater => BinaryOp::Greater,
            Token::GreaterEqual => BinaryOp::GreaterEqual,
            Token::And => BinaryOp::And,
            Token::Or => BinaryOp::Or,
            Token::Xor => BinaryOp::Xor,
            Token::Imp => BinaryOp::Imp,
            Token::Eqv => BinaryOp::Eqv,
            _ => return None,
        })
    }

    pub fn precedence(&self) -> i32 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::Xor => 2,
            BinaryOp::And => 3,
            BinaryOp::Eqv => 4,
            BinaryOp::Imp => 5,
            BinaryOp::Equal | BinaryOp::NotEqual => 6,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => 7,
            BinaryOp::Concat | BinaryOp::Add | BinaryOp::Subtract => 8,
            BinaryOp::Modulo => 9,
            BinaryOp::IntDivide => 10,
            BinaryOp::Multiply | BinaryOp::Divide => 11,
            BinaryOp::Power => 12,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        // Power is right-associative
        !matches!(self, BinaryOp::Power)
    }
}

/// Visitor trait for AST traversal
pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
}
