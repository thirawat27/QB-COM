use crate::opcodes::{ByteCode, OpCode};
use qb_core::data_types::QType;
use qb_core::errors::{QError, QErrorCode, QResult};
use std::collections::HashMap;
use std::io::{self, Write};

/// Virtual Machine for executing QBasic bytecode
pub struct VirtualMachine {
    // Stack-based execution
    value_stack: Vec<QType>,
    call_stack: Vec<usize>,
    instruction_pointer: usize,
    
    // Variable storage
    global_variables: HashMap<String, QType>,
    local_scopes: Vec<HashMap<String, QType>>,
    
    // Arrays storage
    arrays: HashMap<String, Vec<QType>>,
    array_shapes: HashMap<String, Vec<(i32, i32)>>, // (lower, upper) for each dimension
    
    // User-defined type (TYPE...END TYPE) storage: variable -> field -> value
    udt_fields: HashMap<String, HashMap<String, QType>>,
    
    // DATA pointer
    data_pointer: usize,
    
    // Program state
    running: bool,
    error_handler: Option<u32>,
    current_error: Option<QError>,
    
    // Screen mode for graphics
    screen_mode: u8,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            value_stack: Vec::with_capacity(1024),
            call_stack: Vec::with_capacity(256),
            instruction_pointer: 0,
            global_variables: HashMap::new(),
            local_scopes: Vec::new(),
            arrays: HashMap::new(),
            array_shapes: HashMap::new(),
            udt_fields: HashMap::new(),
            data_pointer: 0,
            running: false,
            error_handler: None,
            current_error: None,
            screen_mode: 0,
        }
    }

    pub fn execute(&mut self, bytecode: &ByteCode) -> QResult<()> {
        self.running = true;
        self.instruction_pointer = 0;

        while self.running && self.instruction_pointer < bytecode.len() {
            let op = &bytecode.instructions[self.instruction_pointer];
            
            if let Err(e) = self.execute_instruction(op, bytecode) {
                if let Some(handler) = self.error_handler {
                    self.current_error = Some(e);
                    self.instruction_pointer = handler as usize;
                } else {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn execute_instruction(&mut self, op: &OpCode, bytecode: &ByteCode) -> QResult<()> {
        match op {
            OpCode::Push(value) => {
                self.push(value.clone());
            }
            OpCode::Pop => {
                self.pop()?;
            }
            OpCode::Dup => {
                let val = self.peek()?;
                self.push(val.clone());
            }
            OpCode::Swap => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a);
                self.push(b);
            }

            OpCode::LoadVar(name) => {
                let value = self.get_variable(name)?;
                self.push(value);
            }
            OpCode::StoreVar(name) => {
                let value = self.pop()?;
                self.set_variable(name, value)?;
            }
            OpCode::LoadArray(name, dim_count) => {
                let indices = self.pop_n(*dim_count)?;
                let value = self.get_array_element(name, &indices)?;
                self.push(value);
            }
            OpCode::StoreArray(name, dim_count) => {
                let value = self.pop()?;
                let indices = self.pop_n(*dim_count)?;
                self.set_array_element(name, &indices, value)?;
            }
            OpCode::LoadField(var, field) => {
                let value = self.get_field(var, field)?;
                self.push(value);
            }
            OpCode::StoreField(var, field) => {
                let value = self.pop()?;
                self.set_field(var, field, value)?;
            }
            OpCode::DimArray(name, shape, type_str) => {
                // Calculate total size
                let total_size: usize = shape.iter().map(|(lo, hi)| (hi - lo + 1) as usize).product();
                // Initialize array with appropriate default values based on type
                let default_val = match type_str.as_str() {
                    "INTEGER" => QType::Integer(0),
                    "LONG" => QType::Long(0),
                    "SINGLE" => QType::Single(0.0),
                    "DOUBLE" => QType::Double(0.0),
                    "STRING" => QType::String(String::new()),
                    "_INTEGER64" => QType::Integer64(0),
                    "_UNSIGNED INTEGER" => QType::UnsignedInteger(0),
                    "_UNSIGNED LONG" => QType::UnsignedLong(0),
                    "_UNSIGNED _INTEGER64" => QType::UnsignedInteger64(0),
                    _ => QType::Single(0.0),
                };
                let arr = vec![default_val; total_size];
                self.arrays.insert(name.clone(), arr);
                self.array_shapes.insert(name.clone(), shape.clone());
            }

            OpCode::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.add(&b)?);
            }
            OpCode::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.subtract(&b)?);
            }
            OpCode::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.multiply(&b)?);
            }
            OpCode::Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.divide(&b)?);
            }
            OpCode::IntDiv => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.int_divide(&b)?);
            }
            OpCode::Mod => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.modulo(&b)?);
            }
            OpCode::Pow => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.power(&b)?);
            }
            OpCode::Neg => {
                let a = self.pop()?;
                self.push(a.negate()?);
            }
            OpCode::LogNot => {
                let a = self.pop()?;
                self.push(if self.is_truthy(&a) { QType::Integer(0) } else { QType::Integer(-1) });
            }
            OpCode::LogAnd => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = self.is_truthy(&a) && self.is_truthy(&b);
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::LogOr => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = self.is_truthy(&a) || self.is_truthy(&b);
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }

            OpCode::BitNot => {
                let a = self.pop()?;
                self.push(a.bitwise_not()?);
            }
            OpCode::BitAnd => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bitwise_and(&b)?);
            }
            OpCode::BitOr => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bitwise_or(&b)?);
            }
            OpCode::BitXor => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bitwise_xor(&b)?);
            }
            OpCode::BitImp => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bitwise_imp(&b)?);
            }
            OpCode::BitEqv => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bitwise_eqv(&b)?);
            }

            OpCode::Eq => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Eq)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::Ne => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Ne)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::Lt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Lt)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::Le => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Le)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::Gt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Gt)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }
            OpCode::Ge => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.compare(&b, qb_core::data_types::CompareOp::Ge)?;
                self.push(if result { QType::Integer(-1) } else { QType::Integer(0) });
            }

            OpCode::Jump(addr) => {
                self.instruction_pointer = *addr as usize;
                return Ok(());
            }
            OpCode::JumpIfTrue(addr) => {
                let cond = self.pop()?;
                if self.is_truthy(&cond) {
                    self.instruction_pointer = *addr as usize;
                    return Ok(());
                }
            }
            OpCode::JumpIfFalse(addr) => {
                let cond = self.pop()?;
                if !self.is_truthy(&cond) {
                    self.instruction_pointer = *addr as usize;
                    return Ok(());
                }
            }
            OpCode::Call(addr) => {
                self.call_stack.push(self.instruction_pointer + 1);
                self.instruction_pointer = *addr as usize;
                return Ok(());
            }
            OpCode::Return => {
                if let Some(ret_addr) = self.call_stack.pop() {
                    self.instruction_pointer = ret_addr;
                    return Ok(());
                } else {
                    return Err(QError::runtime(QErrorCode::ReturnWithoutGosub, 0, 0));
                }
            }

            OpCode::Print(newline) => {
                let value = self.pop()?;
                print!("{}", value);
                if *newline {
                    println!();
                }
                io::stdout().flush()?;
            }
            OpCode::PrintComma => {
                // Print tab (move to next 14-column zone)
                print!("\t");
            }
            OpCode::PrintSemicolon => {
                // Do nothing, continue on same line
            }
            OpCode::Input(prompt) => {
                print!("{}", prompt);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let trimmed = input.trim();
                
                // Try to parse as number, otherwise string
                if let Ok(num) = trimmed.parse::<i32>() {
                    self.push(QType::Integer(num as i16));
                } else if let Ok(num) = trimmed.parse::<f64>() {
                    self.push(QType::Double(num));
                } else {
                    self.push(QType::String(trimmed.to_string()));
                }
            }
            OpCode::LineInput(prompt) => {
                print!("{}", prompt);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                self.push(QType::String(input.trim_end().to_string()));
            }
            OpCode::PrintHash(fileno) => {
                // Simplified file output - just print to stdout with prefix
                let value = self.pop()?;
                print!("[#{}]{}", fileno, value);
            }
            OpCode::InputHash(fileno) => {
                // Simplified file input - read from stdin
                print!("[#{}]? ", fileno);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let trimmed = input.trim();
                if let Ok(num) = trimmed.parse::<i32>() {
                    self.push(QType::Integer(num as i16));
                } else if let Ok(num) = trimmed.parse::<f64>() {
                    self.push(QType::Double(num));
                } else {
                    self.push(QType::String(trimmed.to_string()));
                }
            }
            OpCode::Open(filename, mode, fileno) => {
                println!("[OPEN] {} mode={} #{}" , filename, mode, fileno);
            }
            OpCode::Close(fileno) => {
                println!("[CLOSE] #{}" , fileno);
            }
            OpCode::WriteHash(fileno) => {
                let value = self.pop()?;
                print!("[#{}]{},", fileno, value);
            }

            OpCode::Screen(mode) => {
                self.screen_mode = *mode;
                println!("SCREEN {}", mode);
            }
            OpCode::PSet => {
                let _color = self.pop()?;
                let _y = self.pop()?;
                let _x = self.pop()?;
                // Graphics not fully implemented in console mode
            }
            OpCode::PReset => {
                let _y = self.pop()?;
                let _x = self.pop()?;
            }
            OpCode::Line => {
                let _args = self.pop_n(5)?;
                // Not implemented
            }
            OpCode::Circle => {
                let _args = self.pop_n(4)?;
                // Not implemented
            }
            OpCode::Cls => {
                print!("\x1B[2J\x1B[1;1H"); // ANSI clear screen
            }
            OpCode::Color => {
                let _border = self.pop()?;
                let _background = self.pop()?;
                let _foreground = self.pop()?;
                // Color codes not implemented
            }
            OpCode::Locate => {
                let _args = self.pop_n(2)?;
                // Not implemented
            }
            
            // QB64 Graphics extensions (stubs)
            OpCode::RGB(r, g, b) => {
                let color = ((*r as i32) << 16) | ((*g as i32) << 8) | (*b as i32);
                self.push(QType::Long(color));
            }
            OpCode::RGBA(r, g, b, a) => {
                let color = ((*a as i32) << 24) | ((*r as i32) << 16) | ((*g as i32) << 8) | (*b as i32);
                self.push(QType::Long(color));
            }
            OpCode::NewImage(width, height, mode) => {
                println!("[NEWIMAGE] {}x{} mode={}", width, height, mode);
                self.push(QType::Long(1)); // Return image handle
            }
            OpCode::LoadImage(filename) => {
                println!("[LOADIMAGE] {}", filename);
                self.push(QType::Long(1)); // Return image handle
            }
            OpCode::PutImage => {
                let _args = self.pop_n(6)?;
                println!("[PUTIMAGE]");
            }
            
            // QB64 Sound extensions (stubs)
            OpCode::SndOpen(filename) => {
                println!("[SNDOPEN] {}", filename);
                self.push(QType::Long(1)); // Return sound handle
            }
            OpCode::SndClose(handle) => {
                println!("[SNDCLOSE] #{}" , handle);
            }
            OpCode::SndPlay(handle) => {
                println!("[SNDPLAY] #{}" , handle);
            }
            OpCode::SndStop(handle) => {
                println!("[SNDSTOP] #{}" , handle);
            }
            OpCode::SndLoop(handle) => {
                println!("[SNDLOOP] #{}" , handle);
            }
            OpCode::SndVolume(handle, vol) => {
                println!("[SNDVOL] #{} {}" , handle, vol);
            }

            OpCode::Beep => {
                print!("\x07"); // Bell character
            }
            OpCode::Sound => {
                let _duration = self.pop()?;
                let _frequency = self.pop()?;
                // Sound not implemented
            }
            OpCode::Play => {
                let _command = self.pop()?;
                // Play not implemented
            }

            OpCode::Peek => {
                let _addr = self.pop()?;
                self.push(QType::Integer(0)); // Placeholder
            }
            OpCode::Poke => {
                let _value = self.pop()?;
                let _addr = self.pop()?;
                // Not implemented
            }
            OpCode::DefSeg(_seg) => {
                // Not implemented
            }

            OpCode::Concat => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.add(&b)?);
            }
            OpCode::Left => {
                let count = self.pop()?.to_integer()?;
                let s = self.pop()?.to_qstring()?;
                let result: String = s.chars().take(count as usize).collect();
                self.push(QType::String(result));
            }
            OpCode::Right => {
                let count = self.pop()?.to_integer()?;
                let s = self.pop()?.to_qstring()?;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(count as usize);
                let result: String = chars[start..].iter().collect();
                self.push(QType::String(result));
            }
            OpCode::Mid => {
                let len = self.pop()?.to_integer()?;
                let start = self.pop()?.to_integer()?;
                let s = self.pop()?.to_qstring()?;
                let chars: Vec<char> = s.chars().collect();
                let start_idx = (start as usize).saturating_sub(1);
                let result: String = chars[start_idx..]
                    .iter()
                    .take(len as usize)
                    .collect();
                self.push(QType::String(result));
            }
            OpCode::Len => {
                let s = self.pop()?.to_qstring()?;
                self.push(QType::Integer(s.len() as i16));
            }
            OpCode::Asc => {
                let s = self.pop()?.to_qstring()?;
                if let Some(c) = s.chars().next() {
                    self.push(QType::Integer(c as i16));
                } else {
                    return Err(QError::runtime(QErrorCode::IllegalFunctionCall, 0, 0));
                }
            }
            OpCode::Chr => {
                let code = self.pop()?.to_integer()?;
                if let Some(c) = char::from_u32(code as u32) {
                    self.push(QType::String(c.to_string()));
                } else {
                    return Err(QError::runtime(QErrorCode::IllegalFunctionCall, 0, 0));
                }
            }
            OpCode::Str => {
                let n = self.pop()?;
                self.push(QType::String(n.to_string()));
            }
            OpCode::Val => {
                let s = self.pop()?.to_qstring()?;
                if let Ok(n) = s.parse::<f64>() {
                    self.push(QType::Double(n));
                } else {
                    self.push(QType::Double(0.0));
                }
            }
            OpCode::UCase => {
                let s = self.pop()?.to_qstring()?;
                self.push(QType::String(s.to_uppercase()));
            }
            OpCode::LCase => {
                let s = self.pop()?.to_qstring()?;
                self.push(QType::String(s.to_lowercase()));
            }

            OpCode::CInt => {
                let n = self.pop()?;
                self.push(QType::Integer(n.to_integer()?));
            }
            OpCode::CLng => {
                let n = self.pop()?;
                self.push(QType::Long(n.to_long()?));
            }
            OpCode::CSng => {
                let n = self.pop()?;
                self.push(QType::Single(n.to_single()?));
            }
            OpCode::CDbl => {
                let n = self.pop()?;
                self.push(QType::Double(n.to_double()?));
            }
            OpCode::CStr => {
                let n = self.pop()?;
                self.push(QType::String(n.to_qstring()?));
            }

            OpCode::Abs => { let n = self.pop()?; self.push(n.math_abs()?); }
            OpCode::Atn => { let n = self.pop()?; self.push(n.math_atn()?); }
            OpCode::Cos => { let n = self.pop()?; self.push(n.math_cos()?); }
            OpCode::Exp => { let n = self.pop()?; self.push(n.math_exp()?); }
            OpCode::Fix => { let n = self.pop()?; self.push(n.math_fix()?); }
            OpCode::IntOp => { let n = self.pop()?; self.push(n.math_int()?); }
            OpCode::Log => { let n = self.pop()?; self.push(n.math_log()?); }
            OpCode::Rnd => {
                let _n = self.pop()?;
                // Use rand crate to generate a number between 0.0 and 1.0 (exclusive of 1.0)
                let r: f32 = rand::random::<f32>();
                self.push(QType::Single(r));
            }
            OpCode::Sgn => { let n = self.pop()?; self.push(n.math_sgn()?); }
            OpCode::Sin => { let n = self.pop()?; self.push(n.math_sin()?); }
            OpCode::Sqr => { let n = self.pop()?; self.push(n.math_sqr()?); }
            OpCode::Tan => { let n = self.pop()?; self.push(n.math_tan()?); }

            OpCode::EnterScope => {
                self.local_scopes.push(HashMap::new());
            }
            OpCode::ExitScope => {
                self.local_scopes.pop();
            }

            OpCode::Read => {
                if self.data_pointer < bytecode.data_items.len() {
                    let value = bytecode.data_items[self.data_pointer].clone();
                    self.push(value);
                    self.data_pointer += 1;
                } else {
                    return Err(QError::runtime(QErrorCode::OutOfData, 0, 0));
                }
            }
            OpCode::Restore(addr) => {
                self.data_pointer = *addr as usize;
            }

            OpCode::End => {
                self.running = false;
            }
            OpCode::Stop => {
                self.running = false;
            }
            OpCode::Nop => {}
            OpCode::Halt => {
                self.running = false;
            }
            OpCode::PushRet(_) | OpCode::PopRet => {
                // Not fully implemented
            }
        }

        self.instruction_pointer += 1;
        Ok(())
    }

    fn push(&mut self, value: QType) {
        self.value_stack.push(value);
    }

    fn pop(&mut self) -> QResult<QType> {
        self.value_stack.pop().ok_or_else(|| {
            QError::runtime(QErrorCode::OutOfMemory, 0, 0)
        })
    }

    fn pop_n(&mut self, n: usize) -> QResult<Vec<QType>> {
        if self.value_stack.len() < n {
            return Err(QError::runtime(QErrorCode::OutOfMemory, 0, 0));
        }
        let result = self.value_stack.split_off(self.value_stack.len() - n);
        Ok(result)
    }

    fn peek(&self) -> QResult<&QType> {
        self.value_stack.last().ok_or_else(|| {
            QError::runtime(QErrorCode::OutOfMemory, 0, 0)
        })
    }

    fn get_variable(&self, name: &str) -> QResult<QType> {
        // Check local scopes first
        for scope in self.local_scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        // Check global variables
        if let Some(value) = self.global_variables.get(name) {
            return Ok(value.clone());
        }
        // Return default value for undeclared variables
        Ok(QType::Single(0.0))
    }

    fn set_variable(&mut self, name: &str, value: QType) -> QResult<()> {
        // Check if variable exists in any local scope (from innermost to outermost)
        for scope in self.local_scopes.iter_mut().rev() {
            if let Some(v) = scope.get_mut(name) {
                *v = value;
                return Ok(());
            }
        }
        // Check if variable exists in global scope
        if let Some(v) = self.global_variables.get_mut(name) {
            *v = value;
        } else {
            // New variable - set in current local scope if exists, otherwise global
            if let Some(scope) = self.local_scopes.last_mut() {
                scope.insert(name.to_string(), value);
            } else {
                self.global_variables.insert(name.to_string(), value);
            }
        }
        Ok(())
    }

    fn get_field(&self, var: &str, field: &str) -> QResult<QType> {
        if let Some(fields) = self.udt_fields.get(var) {
            if let Some(value) = fields.get(field) {
                return Ok(value.clone());
            }
        }
        // Return default if field doesn't exist
        Ok(QType::Single(0.0))
    }

    fn set_field(&mut self, var: &str, field: &str, value: QType) -> QResult<()> {
        let fields = self.udt_fields.entry(var.to_string()).or_default();
        fields.insert(field.to_string(), value);
        Ok(())
    }

    fn get_array_element(&self, name: &str, indices: &[QType]) -> QResult<QType> {
        if let Some(shape) = self.array_shapes.get(name) {
            if indices.len() != shape.len() {
                return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
            }
            // Calculate flat index using proper stride calculation
            let mut flat_idx = 0usize;
            for (i, (idx, &(lo, hi))) in indices.iter().zip(shape.iter()).enumerate() {
                let idx_val = idx.to_long()?;
                if idx_val < lo || idx_val > hi {
                    return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
                }
                // Calculate stride: product of sizes of all remaining dimensions
                let stride: usize = shape.iter().skip(i + 1)
                    .map(|&(l, h)| (h - l + 1) as usize)
                    .product();
                flat_idx += (idx_val - lo) as usize * stride;
            }
            if let Some(arr) = self.arrays.get(name) {
                if flat_idx < arr.len() {
                    return Ok(arr[flat_idx].clone());
                }
            }
        }
        Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0))
    }

    fn set_array_element(&mut self, name: &str, indices: &[QType], value: QType) -> QResult<()> {
        if let Some(shape) = self.array_shapes.get(name) {
            if indices.len() != shape.len() {
                return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
            }
            // Calculate flat index using proper stride calculation
            let mut flat_idx = 0usize;
            for (i, (idx, &(lo, hi))) in indices.iter().zip(shape.iter()).enumerate() {
                let idx_val = idx.to_long()?;
                if idx_val < lo || idx_val > hi {
                    return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
                }
                // Calculate stride: product of sizes of all remaining dimensions
                let stride: usize = shape.iter().skip(i + 1)
                    .map(|&(l, h)| (h - l + 1) as usize)
                    .product();
                flat_idx += (idx_val - lo) as usize * stride;
            }
            if let Some(arr) = self.arrays.get_mut(name) {
                if flat_idx < arr.len() {
                    arr[flat_idx] = value;
                    return Ok(());
                }
            }
        }
        Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0))
    }

    fn is_truthy(&self, value: &QType) -> bool {
        match value {
            QType::Integer(n) => *n != 0,
            QType::Long(n) => *n != 0,
            QType::Single(n) => *n != 0.0,
            QType::Double(n) => *n != 0.0,
            QType::String(s) => !s.is_empty(),
            _ => false,
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Run bytecode in the VM
pub fn run(bytecode: &ByteCode) -> QResult<()> {
    let mut vm = VirtualMachine::new();
    vm.execute(bytecode)
}
