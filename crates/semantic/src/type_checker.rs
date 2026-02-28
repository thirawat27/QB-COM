use crate::scope::SymbolTable;
use qb_core::data_types::{QType, TypeSuffix};
use qb_core::errors::{QError, QErrorCode, QResult};
use qb_parser::ast_nodes::*;

/// Type checker for QBasic AST
pub struct TypeChecker {
    symbol_table: SymbolTable,
    current_function: Option<String>,
    default_types: [TypeSuffix; 26], // DEFINT A-Z, etc.
}

impl TypeChecker {
    pub fn new() -> Self {
        // Initialize default types (all SINGLE)
        Self {
            symbol_table: SymbolTable::new(),
            current_function: None,
            default_types: [TypeSuffix::Single; 26],
        }
    }

    pub fn check_program(&mut self, program: &Program) -> QResult<()> {
        // First pass: collect all declarations
        for stmt in &program.statements {
            self.collect_declaration(stmt)?;
        }

        // Second pass: type check all statements
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }

        Ok(())
    }

    fn collect_declaration(&mut self, stmt: &Statement) -> QResult<()> {
        match stmt {
            Statement::Dim { vars } => {
                for var in vars {
                    let type_ = self.infer_type_from_spec(&var.type_spec, &var.name);
                    self.symbol_table.define_variable(&var.name.name, type_);
                }
            }
            Statement::Const { name, value } => {
                let type_ = self.infer_type_from_expr(value)?;
                self.symbol_table.define_variable(&name.name, type_);
            }
            Statement::DefType { type_char, letter_range } => {
                let suffix = match type_char {
                    'I' => TypeSuffix::Integer,
                    'L' => TypeSuffix::Long,
                    'S' => TypeSuffix::Single,
                    'D' => TypeSuffix::Double,
                    '$' => TypeSuffix::String,
                    _ => TypeSuffix::Single,
                };
                let start = (letter_range.0.to_ascii_uppercase() as u8 - b'A') as usize;
                let end = (letter_range.1.to_ascii_uppercase() as u8 - b'A') as usize;
                for i in start..=end.min(25) {
                    self.default_types[i] = suffix;
                }
            }
            Statement::Function { name, params, return_type, .. } => {
                let return_qtype = if let Some(spec) = return_type {
                    self.type_spec_to_qtype(spec)
                } else {
                    QType::Single(0.0)
                };
                let param_types = params.iter().map(|_| QType::Single(0.0)).collect();
                self.symbol_table.define_function(name.clone(), param_types, return_qtype);
            }
            Statement::Sub { name, params, .. } => {
                let param_types = params.iter().map(|_| QType::Single(0.0)).collect();
                self.symbol_table.define_subroutine(name.clone(), param_types);
            }
            Statement::LineNumber { number } => {
                self.symbol_table.add_line_number(*number, 0);
            }
            _ => {}
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> QResult<()> {
        match stmt {
            Statement::Assignment { target, value } => {
                let target_type = self.infer_lvalue_type(target)?;
                let value_type = self.infer_type_from_expr(value)?;
                if !self.are_types_compatible(&target_type, &value_type) {
                    return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0));
                }
            }
            Statement::If { condition, then_branch, else_branch, .. } => {
                let cond_type = self.infer_type_from_expr(condition)?;
                if !cond_type.is_numeric() {
                    return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0));
                }
                for s in then_branch {
                    self.check_statement(s)?;
                }
                if let Some(else_stmts) = else_branch {
                    for s in else_stmts {
                        self.check_statement(s)?;
                    }
                }
            }
            Statement::For { var, start, end, step, body } => {
                let var_type = self.infer_type_from_suffix(&var.name);
                for expr in [start, end] {
                    let expr_type = self.infer_type_from_expr(expr)?;
                    if !self.are_types_compatible(&var_type, &expr_type) {
                        return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0));
                    }
                }
                if let Some(step_expr) = step {
                    let step_type = self.infer_type_from_expr(step_expr)?;
                    if !self.are_types_compatible(&var_type, &step_type) {
                        return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0));
                    }
                }
                self.symbol_table.enter_scope();
                self.symbol_table.define_variable(&var.name, var_type);
                for s in body {
                    self.check_statement(s)?;
                }
                self.symbol_table.exit_scope();
            }
            Statement::While { condition, body } | Statement::DoWhile { condition, body } | Statement::DoUntil { condition, body } => {
                let cond_type = self.infer_type_from_expr(condition)?;
                if !cond_type.is_numeric() {
                    return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0));
                }
                self.symbol_table.enter_scope();
                for s in body {
                    self.check_statement(s)?;
                }
                self.symbol_table.exit_scope();
            }
            Statement::DoLoop { body, .. } => {
                self.symbol_table.enter_scope();
                for s in body {
                    self.check_statement(s)?;
                }
                self.symbol_table.exit_scope();
            }
            Statement::Sub { name, body, .. } => {
                self.symbol_table.enter_scope();
                self.symbol_table.define_subroutine(name.clone(), Vec::new());
                for s in body {
                    self.check_statement(s)?;
                }
                self.symbol_table.exit_scope();
            }
            Statement::Function { name, body, .. } => {
                self.current_function = Some(name.clone());
                self.symbol_table.enter_scope();
                for s in body {
                    self.check_statement(s)?;
                }
                self.symbol_table.exit_scope();
                self.current_function = None;
            }
            Statement::Print { items, .. } => {
                for item in items {
                    if let PrintItem::Expression(expr) = item {
                        self.infer_type_from_expr(expr)?;
                    }
                }
            }
            Statement::Input { vars, .. } => {
                for var in vars {
                    if self.symbol_table.lookup_variable(&var.name).is_none() {
                        // Auto-declare input variable with default type
                        let type_ = self.infer_type_from_suffix(&var.name);
                        self.symbol_table.define_variable(&var.name, type_);
                    }
                }
            }
            Statement::Call { name, args } => {
                if self.symbol_table.lookup_subroutine(name).is_none() && 
                   self.symbol_table.lookup_function(name).is_none() {
                    // Allow undefined calls (could be external)
                }
                for arg in args {
                    if let Argument::ByVal(expr) = arg {
                        self.infer_type_from_expr(expr)?;
                    }
                }
            }
            Statement::Goto { label: _ } | Statement::Gosub { label: _ } => {
                // Labels are resolved at runtime
            }
            _ => {
                // Other statements - basic check for now
            }
        }
        Ok(())
    }

    fn infer_lvalue_type(&self, lvalue: &LValue) -> QResult<QType> {
        match lvalue {
            LValue::Variable(var) => {
                if let Some(type_) = self.symbol_table.lookup_variable(&var.name) {
                    Ok(type_.clone())
                } else {
                    // Undeclared variable - use default type
                    Ok(self.infer_type_from_suffix(&var.name))
                }
            }
            LValue::ArrayElement(var, _) => {
                if let Some(type_) = self.symbol_table.lookup_variable(&var.name) {
                    Ok(type_.clone())
                } else {
                    Ok(self.infer_type_from_suffix(&var.name))
                }
            }
            LValue::Field(base, _field) => {
                let _base_type = self.infer_lvalue_type(base)?;
                // TODO: Look up field in user-defined type
                Ok(QType::Single(0.0))
            }
        }
    }

    fn infer_type_from_expr(&self, expr: &Expression) -> QResult<QType> {
        match expr {
            Expression::Integer(_) => Ok(QType::Integer(0)),
            Expression::Long(_) => Ok(QType::Long(0)),
            Expression::Single(_) => Ok(QType::Single(0.0)),
            Expression::Double(_) => Ok(QType::Double(0.0)),
            Expression::String(_) => Ok(QType::String(String::new())),
            Expression::Empty => Ok(QType::Empty),
            Expression::Variable(var) => {
                if let Some(type_) = self.symbol_table.lookup_variable(&var.name) {
                    Ok(type_.clone())
                } else {
                    Ok(self.infer_type_from_suffix(&var.name))
                }
            }
            Expression::ArrayAccess(var, _) => {
                if let Some(type_) = self.symbol_table.lookup_variable(&var.name) {
                    Ok(type_.clone())
                } else {
                    Ok(self.infer_type_from_suffix(&var.name))
                }
            }
            Expression::Negate(e) => self.infer_type_from_expr(e),
            Expression::Not(e) => {
                let t = self.infer_type_from_expr(e)?;
                if t.is_numeric() {
                    Ok(t)
                } else {
                    Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0))
                }
            }
            Expression::Binary { op, left, right } => {
                let left_type = self.infer_type_from_expr(left)?;
                let right_type = self.infer_type_from_expr(right)?;
                self.infer_binary_type(*op, &left_type, &right_type)
            }
            Expression::FunctionCall { name, .. } => {
                if let Some((_, return_type)) = self.symbol_table.lookup_function(name) {
                    Ok(return_type.clone())
                } else {
                    // Built-in function - infer from name
                    self.infer_builtin_function_type(name)
                }
            }
            Expression::TypeConversion { target_type, .. } => {
                self.type_name_to_qtype(target_type)
            }
            Expression::FieldAccess(base, _) => {
                // Simplified - just return type of base for now
                self.infer_type_from_expr(base)
            }
        }
    }

    fn infer_binary_type(&self, op: BinaryOp, left: &QType, right: &QType) -> QResult<QType> {
        match op {
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide |
            BinaryOp::IntDivide | BinaryOp::Modulo | BinaryOp::Power => {
                // Numeric operations
                if left.is_numeric() && right.is_numeric() {
                    // Promote to higher precision
                    if matches!(left, QType::Double(_)) || matches!(right, QType::Double(_)) {
                        Ok(QType::Double(0.0))
                    } else if matches!(left, QType::Single(_)) || matches!(right, QType::Single(_)) {
                        Ok(QType::Single(0.0))
                    } else if matches!(left, QType::Long(_)) || matches!(right, QType::Long(_)) {
                        Ok(QType::Long(0))
                    } else {
                        Ok(QType::Integer(0))
                    }
                } else if matches!(op, BinaryOp::Add) && (left.is_string() || right.is_string()) {
                    // String concatenation
                    Ok(QType::String(String::new()))
                } else {
                    Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0))
                }
            }
            BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Less | BinaryOp::LessEqual |
            BinaryOp::Greater | BinaryOp::GreaterEqual => {
                // Comparison operations return -1 (true) or 0 (false) in QBasic
                Ok(QType::Integer(0))
            }
            BinaryOp::And | BinaryOp::Or | BinaryOp::Xor | BinaryOp::Imp | BinaryOp::Eqv => {
                // Logical operations
                if left.is_numeric() && right.is_numeric() {
                    // Return the wider type
                    if matches!(left, QType::Long(_)) || matches!(right, QType::Long(_)) {
                        Ok(QType::Long(0))
                    } else {
                        Ok(QType::Integer(0))
                    }
                } else {
                    Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0))
                }
            }
            BinaryOp::Concat => {
                Ok(QType::String(String::new()))
            }
        }
    }

    fn infer_type_from_spec(&self, spec: &Option<TypeSpec>, var: &qb_core::data_types::VariableId) -> QType {
        if let Some(spec) = spec {
            self.type_spec_to_qtype(spec)
        } else if let Some(suffix) = &var.suffix {
            self.suffix_to_qtype(suffix)
        } else {
            self.infer_type_from_suffix(&var.name)
        }
    }

    fn infer_type_from_suffix(&self, name: &str) -> QType {
        // Check explicit suffix
        if let Some(last) = name.chars().last() {
            if let Some(suffix) = TypeSuffix::from_char(last) {
                return self.suffix_to_qtype(&suffix);
            }
        }
        // Use default type based on first letter
        if let Some(first) = name.chars().next() {
            if first.is_ascii_alphabetic() {
                let idx = (first.to_ascii_uppercase() as u8 - b'A') as usize;
                if idx < 26 {
                    return self.suffix_to_qtype(&self.default_types[idx]);
                }
            }
        }
        QType::Single(0.0)
    }

    fn suffix_to_qtype(&self, suffix: &TypeSuffix) -> QType {
        match suffix {
            TypeSuffix::Integer => QType::Integer(0),
            TypeSuffix::Long => QType::Long(0),
            TypeSuffix::Single => QType::Single(0.0),
            TypeSuffix::Double => QType::Double(0.0),
            TypeSuffix::String => QType::String(String::new()),
            // QB64 extended types
            TypeSuffix::Integer64 => QType::Integer64(0),
            TypeSuffix::Float => QType::Double(0.0), // Fallback to Double for now
        }
    }

    fn type_spec_to_qtype(&self, spec: &TypeSpec) -> QType {
        match spec {
            TypeSpec::Simple(s) => match s.as_str() {
                "INTEGER" => QType::Integer(0),
                "LONG" => QType::Long(0),
                "SINGLE" => QType::Single(0.0),
                "DOUBLE" => QType::Double(0.0),
                "STRING" => QType::String(String::new()),
                // QB64 extended types
                "_INTEGER64" => QType::Integer64(0),
                "_UNSIGNED INTEGER" => QType::UnsignedInteger(0),
                "_UNSIGNED LONG" => QType::UnsignedLong(0),
                "_UNSIGNED _INTEGER64" => QType::UnsignedInteger64(0),
                _ => QType::Single(0.0),
            }
            TypeSpec::FixedString(_) => QType::String(String::new()),
            TypeSpec::UserDefined(_) => QType::UserDefined(Vec::new()),
        }
    }

    fn type_name_to_qtype(&self, name: &str) -> QResult<QType> {
        match name.to_uppercase().as_str() {
            "INTEGER" => Ok(QType::Integer(0)),
            "LONG" => Ok(QType::Long(0)),
            "SINGLE" => Ok(QType::Single(0.0)),
            "DOUBLE" => Ok(QType::Double(0.0)),
            "STRING" => Ok(QType::String(String::new())),
            // QB64 extended types
            "_INTEGER64" => Ok(QType::Integer64(0)),
            "_UNSIGNED INTEGER" => Ok(QType::UnsignedInteger(0)),
            "_UNSIGNED LONG" => Ok(QType::UnsignedLong(0)),
            "_UNSIGNED _INTEGER64" => Ok(QType::UnsignedInteger64(0)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    fn infer_builtin_function_type(&self, name: &str) -> QResult<QType> {
        let upper = name.to_uppercase();
        match upper.as_str() {
            // Math functions returning numeric
            "ABS" | "ATN" | "COS" | "EXP" | "FIX" | "INT" | "LOG" | "RND" | 
            "SGN" | "SIN" | "SQR" | "TAN" => Ok(QType::Single(0.0)),
            // String functions
            "CHR$" | "DATE$" | "LEFT$" | "LTRIM$" | "MID$" | "RIGHT$" | "RTRIM$" |
            "SPACE$" | "STR$" | "STRING$" | "TIME$" | "TRIM$" | "UCASE$" | "LCASE$" |
            "INKEY$" => Ok(QType::String(String::new())),
            // Integer functions
            "ASC" | "CINT" | "LEN" | "INSTR" | "LBOUND" | "UBOUND" => Ok(QType::Integer(0)),
            "CLNG" | "FREEFILE" => Ok(QType::Long(0)),
            // Type conversion
            "CSNG" => Ok(QType::Single(0.0)),
            "CDBL" => Ok(QType::Double(0.0)),
            "CSTR" => Ok(QType::String(String::new())),
            "VAL" => Ok(QType::Single(0.0)),
            "TIMER" => Ok(QType::Single(0.0)),
            // Memory
            "PEEK" | "INP" => Ok(QType::Integer(0)),
            // File
            "EOF" | "LOF" | "LOC" => Ok(QType::Long(0)),
            // Default
            _ => Ok(QType::Single(0.0)),
        }
    }

    fn are_types_compatible(&self, target: &QType, source: &QType) -> bool {
        match (target, source) {
            (QType::String(_), QType::String(_)) => true,
            (QType::FixedString(_, _), QType::String(_)) => true,
            (QType::String(_), QType::FixedString(_, _)) => true,
            (QType::UserDefined(_), QType::UserDefined(_)) => true,
            (t, s) if t.is_numeric() && s.is_numeric() => true,
            (QType::Empty, _) => true,
            (_, QType::Empty) => true,
            _ => false,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyze a program for semantic errors
pub fn analyze(program: &Program) -> QResult<()> {
    let mut checker = TypeChecker::new();
    checker.check_program(program)
}
