use crate::opcodes::{ByteCode, OpCode};
use qb_core::data_types::QType;
use qb_core::errors::{QError, QErrorCode, QResult};
use std::collections::HashMap;
use qb_parser::ast_nodes::*;

/// Compiles AST to bytecode
pub struct ByteCodeCompiler {
    bytecode: ByteCode,
    label_addresses: HashMap<String, u32>,
    data_label_addresses: HashMap<String, u32>, // For DATA/RESTORE
    pending_jumps: Vec<(usize, String)>, // (instruction_index, label_name)
    current_line: usize,
}

impl ByteCodeCompiler {
    pub fn new() -> Self {
        Self {
            bytecode: ByteCode::new(),
            label_addresses: HashMap::new(),
            data_label_addresses: HashMap::new(),
            pending_jumps: Vec::new(),
            current_line: 1,
        }
    }

    pub fn compile(mut self, program: &Program) -> QResult<ByteCode> {
        // First pass: collect DATA items and their labels
        self.collect_data_labels(program)?;
        
        // Second pass: compile statements - labels are collected during compilation
        for stmt in &program.statements {
            // Collect label at current instruction position (before compiling statement)
            match stmt {
                Statement::Label { name } => {
                    self.label_addresses.insert(name.to_uppercase(), self.bytecode.len() as u32);
                }
                Statement::LineNumber { number } => {
                    self.label_addresses.insert(number.to_string(), self.bytecode.len() as u32);
                }
                _ => {}
            }
            self.compile_statement(stmt)?;
        }

        // Add halt at end
        self.bytecode.emit(OpCode::Halt);

        // Resolve pending jumps
        self.resolve_jumps()?;

        Ok(self.bytecode)
    }
    
    fn collect_data_labels(&mut self, program: &Program) -> QResult<()> {
        for stmt in &program.statements {
            match stmt {
                Statement::Label { name } => {
                    // Store current data pointer position for this label
                    self.data_label_addresses.insert(name.to_uppercase(), self.bytecode.data_items.len() as u32);
                }
                Statement::LineNumber { number } => {
                    // Store current data pointer position for this line number
                    self.data_label_addresses.insert(number.to_string(), self.bytecode.data_items.len() as u32);
                }
                Statement::Data { values } => {
                    // Add data items and track the index
                    for val in values {
                        match val {
                            Expression::Integer(n) => {
                                if *n >= i16::MIN as i32 && *n <= i16::MAX as i32 {
                                    self.bytecode.add_data(QType::Integer(*n as i16))
                                } else {
                                    self.bytecode.add_data(QType::Long(*n))
                                }
                            }
                            Expression::Long(n) => {
                                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                                    self.bytecode.add_data(QType::Long(*n as i32))
                                } else {
                                    self.bytecode.add_data(QType::Integer64(*n))
                                }
                            }
                            Expression::Single(n) => self.bytecode.add_data(QType::Single(*n)),
                            Expression::Double(n) => self.bytecode.add_data(QType::Double(*n)),
                            Expression::String(s) => self.bytecode.add_data(QType::String(s.clone())),
                            _ => {} // Only literals in DATA
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn resolve_jumps(&mut self) -> QResult<()> {
        for (idx, label) in &self.pending_jumps {
            if let Some(&addr) = self.label_addresses.get(&label.to_uppercase()) {
                // Update the jump instruction
                match self.bytecode.instructions[*idx] {
                    OpCode::Jump(_) => {
                        self.bytecode.instructions[*idx] = OpCode::Jump(addr);
                    }
                    OpCode::JumpIfTrue(_) => {
                        self.bytecode.instructions[*idx] = OpCode::JumpIfTrue(addr);
                    }
                    OpCode::JumpIfFalse(_) => {
                        self.bytecode.instructions[*idx] = OpCode::JumpIfFalse(addr);
                    }
                    OpCode::Call(_) => {
                        self.bytecode.instructions[*idx] = OpCode::Call(addr);
                    }
                    _ => {}
                }
            } else {
                return Err(QError::runtime(
                    QErrorCode::LabelNotDefined,
                    self.current_line,
                    0,
                ));
            }
        }
        Ok(())
    }

    fn compile_statement(&mut self, stmt: &Statement) -> QResult<()> {
        match stmt {
            Statement::Rem(_) => {
                // Comments are ignored
            }
            Statement::Dim { vars } => {
                for var in vars {
                    // Check if it's an array
                    if let Some(ref bounds) = var.bounds {
                        // Array - emit DimArray opcode with shape and type
                        let shape: Vec<(i32, i32)> = bounds.iter().map(|b| (b.lower, b.upper)).collect();
                        let type_str = if let Some(ref spec) = var.type_spec {
                            match spec {
                                TypeSpec::Simple(s) => s.clone(),
                                _ => "SINGLE".to_string(),
                            }
                        } else {
                            "SINGLE".to_string()
                        };
                        self.bytecode.emit(OpCode::DimArray(var.name.full_name(), shape, type_str));
                    } else {
                        // Scalar variable - Initialize with default value
                        let type_ = if let Some(ref spec) = var.type_spec {
                            self.type_spec_to_qtype(spec)
                        } else {
                            QType::Single(0.0)
                        };
                        self.bytecode.emit(OpCode::Push(type_.default_value()));
                        self.bytecode.emit(OpCode::StoreVar(var.name.full_name()));
                    }
                }
            }
            Statement::Const { name, value } => {
                // Initialize constant
                self.compile_expression(value)?;
                self.bytecode.emit(OpCode::StoreVar(name.full_name()));
            }
            Statement::Assignment { target, value } => {
                match target {
                    LValue::Variable(var) => {
                        self.compile_expression(value)?;
                        self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                    }
                    LValue::ArrayElement(var, indices) => {
                        // For array: compile indices first, then value
                        for idx in indices {
                            self.compile_expression(idx)?;
                        }
                        self.compile_expression(value)?;
                        self.bytecode.emit(OpCode::StoreArray(var.full_name(), indices.len()));
                    }
                    LValue::Field(var, field) => {
                        // Get the base variable name from the LValue
                        let base_name = self.lvalue_to_string(var);
                        self.bytecode.emit(OpCode::StoreField(base_name, field.clone()));
                    }
                }
            }
            Statement::If { condition, then_branch, else_branch, .. } => {
                self.compile_expression(condition)?;
                
                let jump_if_false_idx = self.bytecode.len();
                self.bytecode.emit(OpCode::JumpIfFalse(0)); // Placeholder
                
                for s in then_branch {
                    self.compile_statement(s)?;
                }
                
                if let Some(else_stmts) = else_branch {
                    let jump_over_else_idx = self.bytecode.len();
                    self.bytecode.emit(OpCode::Jump(0)); // Placeholder
                    
                    let else_start = self.bytecode.len() as u32;
                    self.bytecode.instructions[jump_if_false_idx] = OpCode::JumpIfFalse(else_start);
                    
                    for s in else_stmts {
                        self.compile_statement(s)?;
                    }
                    
                    let after_else = self.bytecode.len() as u32;
                    self.bytecode.instructions[jump_over_else_idx] = OpCode::Jump(after_else);
                } else {
                    let after_then = self.bytecode.len() as u32;
                    self.bytecode.instructions[jump_if_false_idx] = OpCode::JumpIfFalse(after_then);
                }
            }
            Statement::Select { expr, cases, case_else } => {
                // Evaluate the select expression and push it to stack
                self.compile_expression(expr)?;
                
                let mut end_jumps = Vec::new();
                let mut next_case_jump = None;
                
                for case in cases {
                    if let Some(idx) = next_case_jump {
                        let current_idx = self.bytecode.len() as u32;
                        self.bytecode.instructions[idx] = OpCode::JumpIfFalse(current_idx);
                    }
                    
                    // Evaluate case conditions (combined with OR)
                    // For now, only simple Expressions are handled exactly (Is and Range omitted for simplicity)
                    let mut first = true;
                    for cond in &case.conditions {
                        if !first {
                            self.bytecode.emit(OpCode::LogOr);
                        }
                        
                        match cond {
                            CaseCondition::Expression(e) => {
                                self.bytecode.emit(OpCode::Dup); // Duplicate 'expr' to compare
                                self.compile_expression(e)?;
                                self.bytecode.emit(OpCode::Eq);
                            }
                            CaseCondition::Range(start, end) => {
                                // expr >= start AND expr <= end
                                self.bytecode.emit(OpCode::Dup);
                                self.compile_expression(start)?;
                                self.bytecode.emit(OpCode::Ge);
                                
                                self.bytecode.emit(OpCode::Dup);
                                self.compile_expression(end)?;
                                self.bytecode.emit(OpCode::Le);
                                
                                self.bytecode.emit(OpCode::LogAnd);
                            }
                            CaseCondition::Is(op_tok, e) => {
                                self.bytecode.emit(OpCode::Dup);
                                self.compile_expression(e)?;
                                if let Some(op) = BinaryOp::from_token(op_tok) {
                                    self.compile_binary_op(op)?;
                                } else {
                                    self.bytecode.emit(OpCode::Eq); // Fallback
                                }
                            }
                        }
                        first = false;
                    }
                    
                    let false_jump = self.bytecode.len();
                    self.bytecode.emit(OpCode::JumpIfFalse(0)); // Jump to next case
                    next_case_jump = Some(false_jump);
                    
                    // Case body
                    for s in &case.body {
                        self.compile_statement(s)?;
                    }
                    
                    // Jump to end of select
                    let end_jump = self.bytecode.len();
                    self.bytecode.emit(OpCode::Jump(0));
                    end_jumps.push(end_jump);
                }
                
                if let Some(idx) = next_case_jump {
                    let current_idx = self.bytecode.len() as u32;
                    self.bytecode.instructions[idx] = OpCode::JumpIfFalse(current_idx);
                }
                
                if let Some(else_stmts) = case_else {
                    for s in else_stmts {
                        self.compile_statement(s)?;
                    }
                }
                
                let end_idx = self.bytecode.len() as u32;
                for idx in end_jumps {
                    self.bytecode.instructions[idx] = OpCode::Jump(end_idx);
                }
                
                self.bytecode.emit(OpCode::Pop); // Pop the select expression
            }
            Statement::For { var, start, end, step, body } => {
                // Initialize loop variable
                self.compile_expression(start)?;
                self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                
                let loop_start = self.bytecode.len() as u32;
                
                // Check condition based on step direction
                self.bytecode.emit(OpCode::LoadVar(var.full_name()));
                self.compile_expression(end)?;
                
                // Determine comparison operator based on step value
                let is_negative_step = step.as_ref().map(|s| {
                    matches!(s, Expression::Integer(n) if *n < 0) ||
                    matches!(s, Expression::Long(n) if *n < 0) ||
                    matches!(s, Expression::Single(n) if *n < 0.0) ||
                    matches!(s, Expression::Double(n) if *n < 0.0)
                }).unwrap_or(false);
                
                if is_negative_step {
                    self.bytecode.emit(OpCode::Ge); // >= for negative step (counting down)
                } else {
                    self.bytecode.emit(OpCode::Le); // <= for positive step (counting up)
                }
                
                let exit_jump_idx = self.bytecode.len();
                self.bytecode.emit(OpCode::JumpIfFalse(0)); // Placeholder
                
                // Compile body
                for s in body {
                    self.compile_statement(s)?;
                }
                
                // Increment
                self.bytecode.emit(OpCode::LoadVar(var.full_name()));
                if let Some(step_expr) = step {
                    self.compile_expression(step_expr)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(1)));
                }
                self.bytecode.emit(OpCode::Add);
                self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                
                // Jump back
                self.bytecode.emit(OpCode::Jump(loop_start));
                
                // Update exit jump
                let after_loop = self.bytecode.len() as u32;
                self.bytecode.instructions[exit_jump_idx] = OpCode::JumpIfFalse(after_loop);
            }
            Statement::While { condition, body } => {
                let loop_start = self.bytecode.len() as u32;
                
                self.compile_expression(condition)?;
                let exit_jump_idx = self.bytecode.len();
                self.bytecode.emit(OpCode::JumpIfFalse(0)); // Placeholder
                
                for s in body {
                    self.compile_statement(s)?;
                }
                
                self.bytecode.emit(OpCode::Jump(loop_start));
                
                let after_loop = self.bytecode.len() as u32;
                self.bytecode.instructions[exit_jump_idx] = OpCode::JumpIfFalse(after_loop);
            }
            Statement::DoWhile { condition, body } => {
                let loop_start = self.bytecode.len() as u32;
                
                self.compile_expression(condition)?;
                let exit_jump_idx = self.bytecode.len();
                self.bytecode.emit(OpCode::JumpIfFalse(0)); // Placeholder
                
                for s in body {
                    self.compile_statement(s)?;
                }
                
                self.bytecode.emit(OpCode::Jump(loop_start));
                
                let after_loop = self.bytecode.len() as u32;
                self.bytecode.instructions[exit_jump_idx] = OpCode::JumpIfFalse(after_loop);
            }
            Statement::DoUntil { condition, body } => {
                let loop_start = self.bytecode.len() as u32;
                
                self.compile_expression(condition)?;
                let exit_jump_idx = self.bytecode.len();
                self.bytecode.emit(OpCode::JumpIfTrue(0)); // Placeholder
                
                for s in body {
                    self.compile_statement(s)?;
                }
                
                self.bytecode.emit(OpCode::Jump(loop_start));
                
                let after_loop = self.bytecode.len() as u32;
                self.bytecode.instructions[exit_jump_idx] = OpCode::JumpIfTrue(after_loop);
            }
            Statement::Goto { label } => {
                let idx = self.bytecode.len();
                self.bytecode.emit(OpCode::Jump(0)); // Placeholder
                self.pending_jumps.push((idx, label.clone()));
            }
            Statement::Gosub { label } => {
                let idx = self.bytecode.len();
                self.bytecode.emit(OpCode::Call(0)); // Placeholder
                self.pending_jumps.push((idx, label.clone()));
            }
            Statement::Return => {
                self.bytecode.emit(OpCode::Return);
            }
            Statement::Print { items, .. } => {
                let mut needs_newline = true;
                
                for item in items.iter() {
                    match item {
                        PrintItem::Expression(expr) => {
                            self.compile_expression(expr)?;
                            self.bytecode.emit(OpCode::Print(false));
                            needs_newline = true;
                        }
                        PrintItem::Semicolon => {
                            needs_newline = false;
                        }
                        PrintItem::Comma => {
                            self.bytecode.emit(OpCode::PrintComma);
                            needs_newline = false;
                        }
                    }
                }
                
                if needs_newline {
                    self.bytecode.emit(OpCode::Push(QType::String(String::new())));
                    self.bytecode.emit(OpCode::Print(true));
                }
            }
            Statement::Input { prompt, vars } => {
                let prompt_str = prompt.clone().unwrap_or_else(|| "? ".to_string());
                for var in vars {
                    self.bytecode.emit(OpCode::Input(prompt_str.clone()));
                    self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                }
            }
            Statement::LineInput { prompt, var } => {
                let prompt_str = prompt.clone().unwrap_or_default();
                self.bytecode.emit(OpCode::LineInput(prompt_str));
                self.bytecode.emit(OpCode::StoreVar(var.full_name()));
            }
            Statement::Open { filename, mode, fileno, .. } => {
                // Simple file open: evaluate filename, mode, fileno
                if let Expression::String(fname) = filename {
                    let mode_str = format!("{:?}", mode);
                    let fileno_val = if let Expression::Integer(n) = fileno { *n as u8 } else { 1 };
                    self.bytecode.emit(OpCode::Open(fname.clone(), mode_str, fileno_val));
                }
            }
            Statement::Close { fileno } => {
                let fileno_val = if let Some(Expression::Integer(n)) = fileno { *n as u8 } else { 0 };
                self.bytecode.emit(OpCode::Close(fileno_val));
            }
            Statement::PrintHash { fileno, items } => {
                let fileno_val = if let Expression::Integer(n) = fileno { *n as u8 } else { 1 };
                for item in items {
                    match item {
                        PrintItem::Expression(expr) => {
                            self.compile_expression(expr)?;
                        }
                        PrintItem::Comma => {
                            self.bytecode.emit(OpCode::PrintComma);
                        }
                        PrintItem::Semicolon => {
                            self.bytecode.emit(OpCode::PrintSemicolon);
                        }
                    }
                }
                self.bytecode.emit(OpCode::PrintHash(fileno_val));
            }
            Statement::InputHash { fileno, vars } => {
                let fileno_val = if let Expression::Integer(n) = fileno { *n as u8 } else { 1 };
                for var in vars {
                    self.bytecode.emit(OpCode::InputHash(fileno_val));
                    self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                }
            }
            Statement::Call { name, args } => {
                for arg in args {
                    if let Argument::ByVal(expr) = arg {
                        self.compile_expression(expr)?;
                    }
                }
                // For now, treat as label call
                let idx = self.bytecode.len();
                self.bytecode.emit(OpCode::Call(0)); // Placeholder
                self.pending_jumps.push((idx, name.clone()));
            }
            Statement::Screen { mode: Expression::Integer(m) } => {
                self.bytecode.emit(OpCode::Screen(*m as u8));
            }
            Statement::PSet { x, y, color } => {
                self.compile_expression(x)?;
                self.compile_expression(y)?;
                if let Some(c) = color {
                    self.compile_expression(c)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                self.bytecode.emit(OpCode::PSet);
            }
            Statement::PReset { x, y } => {
                self.compile_expression(x)?;
                self.compile_expression(y)?;
                self.bytecode.emit(OpCode::PReset);
            }
            Statement::Cls => {
                self.bytecode.emit(OpCode::Cls);
            }
            Statement::Color { foreground, background, border } => {
                if let Some(fg) = foreground {
                    self.compile_expression(fg)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                if let Some(bg) = background {
                    self.compile_expression(bg)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                if let Some(bd) = border {
                    self.compile_expression(bd)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                self.bytecode.emit(OpCode::Color);
            }
            Statement::Beep => {
                self.bytecode.emit(OpCode::Beep);
            }
            Statement::Sound { frequency, duration } => {
                self.compile_expression(frequency)?;
                self.compile_expression(duration)?;
                self.bytecode.emit(OpCode::Sound);
            }
            Statement::End => {
                self.bytecode.emit(OpCode::End);
            }
            Statement::Stop => {
                self.bytecode.emit(OpCode::Stop);
            }
            Statement::Label { .. } | Statement::LineNumber { .. } => {
                // Labels are handled during collection
            }
            Statement::Data { .. } => {
                // DATA statements are processed in collect_data_labels, nothing to do here
            }
            Statement::Read { vars } => {
                for var in vars {
                    self.bytecode.emit(OpCode::Read);
                    self.bytecode.emit(OpCode::StoreVar(var.full_name()));
                }
            }
            Statement::Restore { label } => {
                if let Some(lbl) = label {
                    if let Some(&addr) = self.data_label_addresses.get(&lbl.to_uppercase()) {
                        self.bytecode.emit(OpCode::Restore(addr));
                    } else {
                        // Label not found, restore to beginning
                        self.bytecode.emit(OpCode::Restore(0));
                    }
                } else {
                    self.bytecode.emit(OpCode::Restore(0)); // Restore to beginning
                }
            }
            Statement::Line { x1, y1, x2, y2, color, style: _, is_box: _, is_filled: _ } => {
                self.compile_expression(x1)?;
                self.compile_expression(y1)?;
                self.compile_expression(x2)?;
                self.compile_expression(y2)?;
                if let Some(c) = color {
                    self.compile_expression(c)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                self.bytecode.emit(OpCode::Line);
            }
            Statement::Circle { x, y, radius, color, start: _, end: _, aspect: _ } => {
                self.compile_expression(x)?;
                self.compile_expression(y)?;
                self.compile_expression(radius)?;
                if let Some(c) = color {
                    self.compile_expression(c)?;
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer(-1)));
                }
                self.bytecode.emit(OpCode::Circle);
            }
            Statement::Locate { row, col, cursor: _, start: _, stop: _ } => {
                // Optional arguments push -1 if omitted
                if let Some(r) = row { self.compile_expression(r)?; } else { self.bytecode.emit(OpCode::Push(QType::Integer(-1))); }
                if let Some(c) = col { self.compile_expression(c)?; } else { self.bytecode.emit(OpCode::Push(QType::Integer(-1))); }
                self.bytecode.emit(OpCode::Locate);
            }
            _ => {
                // Other statements not yet implemented
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expression) -> QResult<()> {
        match expr {
            Expression::Integer(n) => {
                // Use Integer (i16) for small values, Long (i32) for larger values
                if *n >= i16::MIN as i32 && *n <= i16::MAX as i32 {
                    self.bytecode.emit(OpCode::Push(QType::Integer(*n as i16)));
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Long(*n)));
                }
            }
            Expression::Long(n) => {
                // Check if value fits in i32 (QB LONG), otherwise use Integer64
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    self.bytecode.emit(OpCode::Push(QType::Long(*n as i32)));
                } else {
                    self.bytecode.emit(OpCode::Push(QType::Integer64(*n)));
                }
            }
            Expression::Single(n) => {
                self.bytecode.emit(OpCode::Push(QType::Single(*n)));
            }
            Expression::Double(n) => {
                self.bytecode.emit(OpCode::Push(QType::Double(*n)));
            }
            Expression::String(s) => {
                self.bytecode.emit(OpCode::Push(QType::String(s.clone())));
            }
            Expression::Variable(var) => {
                self.bytecode.emit(OpCode::LoadVar(var.full_name()));
            }
            Expression::ArrayAccess(var, indices) => {
                for idx in indices {
                    self.compile_expression(idx)?;
                }
                self.bytecode.emit(OpCode::LoadArray(var.full_name(), indices.len()));
            }
            Expression::Negate(e) => {
                self.compile_expression(e)?;
                self.bytecode.emit(OpCode::Neg);
            }
            Expression::Not(e) => {
                self.compile_expression(e)?;
                self.bytecode.emit(OpCode::BitNot);
            }
            Expression::Binary { op, left, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                self.compile_binary_op(*op)?;
            }
            Expression::FunctionCall { name, args } => {
                for arg in args {
                    self.compile_expression(arg)?;
                }
                self.compile_builtin_function(name, args.len())?;
            }
            Expression::TypeConversion { target_type, expr } => {
                self.compile_expression(expr)?;
                self.compile_conversion(target_type)?;
            }
            Expression::FieldAccess(expr, field) => {
                // For now, assume expr is a variable
                if let Expression::Variable(var) = expr.as_ref() {
                    self.bytecode.emit(OpCode::LoadField(var.full_name(), field.clone()));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_binary_op(&mut self, op: BinaryOp) -> QResult<()> {
        let opcode = match op {
            BinaryOp::Add => OpCode::Add,
            BinaryOp::Subtract => OpCode::Sub,
            BinaryOp::Multiply => OpCode::Mul,
            BinaryOp::Divide => OpCode::Div,
            BinaryOp::IntDivide => OpCode::IntDiv,
            BinaryOp::Modulo => OpCode::Mod,
            BinaryOp::Power => OpCode::Pow,
            BinaryOp::Concat => OpCode::Concat,
            BinaryOp::Equal => OpCode::Eq,
            BinaryOp::NotEqual => OpCode::Ne,
            BinaryOp::Less => OpCode::Lt,
            BinaryOp::LessEqual => OpCode::Le,
            BinaryOp::Greater => OpCode::Gt,
            BinaryOp::GreaterEqual => OpCode::Ge,
            BinaryOp::And => OpCode::BitAnd,
            BinaryOp::Or => OpCode::BitOr,
            BinaryOp::Xor => OpCode::BitXor,
            BinaryOp::Imp => OpCode::BitImp,
            BinaryOp::Eqv => OpCode::BitEqv,
        };
        self.bytecode.emit(opcode);
        Ok(())
    }

    fn compile_builtin_function(&mut self, name: &str, _arg_count: usize) -> QResult<()> {
        let upper = name.to_uppercase();
        let opcode = match upper.as_str() {
            "ABS" => OpCode::Abs,
            "ATN" => OpCode::Atn,
            "COS" => OpCode::Cos,
            "EXP" => OpCode::Exp,
            "FIX" => OpCode::Fix,
            "INT" => OpCode::IntOp,
            "LOG" => OpCode::Log,
            "RND" => OpCode::Rnd,
            "SGN" => OpCode::Sgn,
            "SIN" => OpCode::Sin,
            "SQR" => OpCode::Sqr,
            "TAN" => OpCode::Tan,
            "CHR$" => OpCode::Chr,
            "LEFT$" => OpCode::Left,
            "RIGHT$" => OpCode::Right,
            "MID$" => OpCode::Mid,
            "LEN" => OpCode::Len,
            "ASC" => OpCode::Asc,
            "STR$" => OpCode::Str,
            "VAL" => OpCode::Val,
            "UCASE" | "UCASE$" => OpCode::UCase,
            "LCASE" | "LCASE$" => OpCode::LCase,
            "CINT" => OpCode::CInt,
            "CLNG" => OpCode::CLng,
            "CSNG" => OpCode::CSng,
            "CDBL" => OpCode::CDbl,
            "CSTR" => OpCode::CStr,
            _ => OpCode::Nop,
        };
        self.bytecode.emit(opcode);
        Ok(())
    }

    fn compile_conversion(&mut self, target_type: &str) -> QResult<()> {
        let opcode = match target_type.to_uppercase().as_str() {
            "INTEGER" => OpCode::CInt,
            "LONG" => OpCode::CLng,
            "SINGLE" => OpCode::CSng,
            "DOUBLE" => OpCode::CDbl,
            "STRING" => OpCode::CStr,
            _ => OpCode::Nop,
        };
        self.bytecode.emit(opcode);
        Ok(())
    }

    fn lvalue_to_string(&self, lval: &LValue) -> String {
        match lval {
            LValue::Variable(var) => var.full_name(),
            LValue::ArrayElement(var, _) => var.full_name(),
            LValue::Field(inner, field) => {
                format!("{}.{}", self.lvalue_to_string(inner), field)
            }
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
}

impl Default for ByteCodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile a program to bytecode
pub fn compile(program: &Program) -> QResult<ByteCode> {
    let compiler = ByteCodeCompiler::new();
    compiler.compile(program)
}
