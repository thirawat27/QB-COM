use crate::ast_nodes::*;
use crate::declarations::DeclarationManager;
use qb_core::data_types::{ArrayBounds, ParamType};
use qb_core::errors::{QError, QResult};
use qb_lexer::tokens::{Token, TokenInfo};


/// Recursive descent parser for QBasic
pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    declaration_manager: DeclarationManager,
    in_sub: bool,
    in_function: bool,
    in_loop: bool,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens,
            current: 0,
            declaration_manager: DeclarationManager::new(),
            in_sub: false,
            in_function: false,
            in_loop: false,
        }
    }

    pub fn parse(mut self) -> QResult<Program> {
        let mut program = Program::new();

        while !self.is_at_end() {
            // Skip newlines
            self.skip_newlines();
            
            if self.is_at_end() {
                break;
            }

            // Check for line number
            if let Some(Token::LineNumber(n)) = self.peek_token() {
                let num = *n;
                self.advance();
                program.add_statement(Statement::LineNumber { number: num });
                program.line_numbers.insert(num, program.statements.len() - 1);
            }

            let stmt = self.parse_statement()?;
            // Skip empty REM statements (from newlines)
            if !matches!(stmt, Statement::Rem(ref s) if s.is_empty()) {
                program.add_statement(stmt);
            }
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> QResult<Statement> {
        match self.peek_token() {
            Some(Token::Rem) => {
                self.advance();
                let comment = if let Some(Token::String(s)) = self.peek_token() {
                    let s = s.clone();
                    self.advance();
                    s
                } else {
                    String::new()
                };
                Ok(Statement::Rem(comment))
            }
            Some(Token::Dim) => self.parse_dim(),
            Some(Token::Const) => self.parse_const(),
            Some(Token::DefInt) | Some(Token::DefLng) | Some(Token::DefSng) | 
            Some(Token::DefDbl) | Some(Token::DefStr) => self.parse_deftype(),
            Some(Token::Type) => self.parse_type_def(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Select) => self.parse_select(),
            Some(Token::For) => self.parse_for(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Do) => self.parse_do(),
            Some(Token::GoTo) => self.parse_goto(),
            Some(Token::GoSub) => self.parse_gosub(),
            Some(Token::Return) => {
                self.advance();
                Ok(Statement::Return)
            }
            Some(Token::On) => self.parse_on(),
            Some(Token::Sub) => self.parse_sub(),
            Some(Token::Function) => self.parse_function(),
            Some(Token::Declare) => self.parse_declare(),
            Some(Token::Call) => self.parse_call(),
            Some(Token::Exit) => self.parse_exit(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::PrintHash) => self.parse_print_hash(),
            Some(Token::Input) => self.parse_input(),
            Some(Token::InputHash) => self.parse_input_hash(),
            Some(Token::LineInput) => self.parse_line_input(),
            Some(Token::Write) => self.parse_write(),
            Some(Token::Open) => self.parse_open(),
            Some(Token::Close) => self.parse_close(),
            Some(Token::Get) => self.parse_get(),
            Some(Token::Put) => self.parse_put(),
            Some(Token::Seek) => self.parse_seek(),
            Some(Token::Lock) => self.parse_lock(),
            Some(Token::Unlock) => self.parse_unlock(),
            Some(Token::Screen) => self.parse_screen(),
            Some(Token::PSet) => self.parse_pset(),
            Some(Token::PReset) => self.parse_preset(),
            Some(Token::Line) => self.parse_line(),
            Some(Token::Circle) => self.parse_circle(),
            Some(Token::Draw) => self.parse_draw(),
            Some(Token::Paint) => self.parse_paint(),
            Some(Token::View) => self.parse_view(),
            Some(Token::Window) => self.parse_window(),
            Some(Token::Palette) => self.parse_palette(),
            Some(Token::Color) => self.parse_color(),
            Some(Token::Cls) => {
                self.advance();
                Ok(Statement::Cls)
            }
            Some(Token::Locate) => self.parse_locate(),
            Some(Token::Width) => self.parse_width(),
            Some(Token::Beep) => {
                self.advance();
                Ok(Statement::Beep)
            }
            Some(Token::Sound) => self.parse_sound(),
            Some(Token::Play) => self.parse_play(),
            Some(Token::Poke) => self.parse_poke(),
            Some(Token::DefSeg) => self.parse_defseg(),
            Some(Token::Randomize) => self.parse_randomize(),
            Some(Token::Data) => self.parse_data(),
            Some(Token::Read) => self.parse_read(),
            Some(Token::Restore) => self.parse_restore(),
            Some(Token::Environ) => self.parse_environ(),
            Some(Token::Shell) => self.parse_shell(),
            Some(Token::System) => {
                self.advance();
                Ok(Statement::System)
            }
            Some(Token::OnError) => self.parse_on_error(),
            Some(Token::Resume) => self.parse_resume(),
            Some(Token::Error) => self.parse_error(),
            // QB64 Metacommands (treated as comments/ignored for now)
            Some(Token::MetaDynamic) | Some(Token::MetaStatic) | Some(Token::MetaConsole) |
            Some(Token::MetaResize) | Some(Token::MetaScreenShow) | Some(Token::ScreenHide) => {
                self.advance();
                Ok(Statement::Rem(format!("Metacommand: {:?}", self.peek_token())))
            }
            Some(Token::MetaInclude) => {
                self.advance();
                // Skip the include path
                if let Some(Token::String(_)) = self.peek_token() {
                    self.advance();
                }
                Ok(Statement::Rem(String::from("$INCLUDE")))
            }
            Some(Token::MetaIf) | Some(Token::MetaElse) | Some(Token::MetaEndIf) => {
                self.advance();
                Ok(Statement::Rem(format!("Metacommand: {:?}", self.peek_token())))
            }
            Some(Token::End) => {
                self.advance();
                // Check for END TYPE, END SUB, END FUNCTION, etc.
                match self.peek_token() {
                    Some(Token::Type) => {
                        self.advance();
                        Ok(Statement::Rem(String::from("END TYPE")))
                    }
                    Some(Token::Sub) => {
                        self.advance();
                        self.in_sub = false;
                        Ok(Statement::ExitSub)
                    }
                    Some(Token::Function) => {
                        self.advance();
                        self.in_function = false;
                        Ok(Statement::ExitFunction)
                    }
                    Some(Token::If) => {
                        self.advance();
                        Ok(Statement::Rem(String::from("END IF")))
                    }
                    Some(Token::Select) => {
                        self.advance();
                        Ok(Statement::Rem(String::from("END SELECT")))
                    }
                    _ => Ok(Statement::End),
                }
            }
            Some(Token::Stop) => {
                self.advance();
                Ok(Statement::Stop)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                
                if self.check(Token::Colon) {
                    self.advance();
                    Ok(Statement::Label { name })
                } else {
                    self.parse_identifier_statement(&name)
                }
            }
            Some(Token::Label(label)) => {
                let label = label.clone();
                self.advance();
                Ok(Statement::Label { name: label })
            }
            Some(Token::Let) => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.peek_token() {
                    let name = name.clone();
                    self.advance();
                    self.parse_identifier_statement(&name)
                } else {
                    let (line, col) = self.current_pos();
                    Err(QError::compile("Expected identifier after LET", line, col))
                }
            }
            Some(Token::NewLine) => {
                self.advance();
                Ok(Statement::Rem(String::new()))
            }
            _ => {
                let (line, col) = self.current_pos();
                Err(QError::compile(
                    format!("Unexpected token: {:?}", self.peek_token()),
                    line,
                    col
                ))
            }
        }
    }

    fn parse_identifier_statement(&mut self, name: &str) -> QResult<Statement> {
        // Check for assignment or procedure call
        if self.check(Token::Equal) {
            // Simple assignment
            self.advance();
            let value = self.parse_expression()?;
            Ok(Statement::Assignment {
                target: LValue::Variable(qb_core::data_types::VariableId::new(name, None)),
                value,
            })
        } else if self.check(Token::LParen) {
            // Array element assignment or function call
            let indices = self.parse_array_indices()?;
            
            if self.check(Token::Equal) {
                self.advance();
                let value = self.parse_expression()?;
                Ok(Statement::Assignment {
                    target: LValue::ArrayElement(
                        qb_core::data_types::VariableId::new(name, None),
                        indices
                    ),
                    value,
                })
            } else {
                // Function call statement (without CALL)
                let mut args = indices;
                if self.check(Token::Comma) {
                    self.advance();
                    args.extend(self.parse_argument_list()?);
                }
                Ok(Statement::Call {
                    name: name.to_string(),
                    args: args.into_iter().map(Argument::ByVal).collect(),
                })
            }
        } else {
            // Simple procedure call or variable assignment without LET
            let line = self.current_line();
            let col = self.current_column();
            Err(QError::compile(
                format!("Expected '=' or '(' after identifier '{}'", name),
                line,
                col
            ))
        }
    }

    // ... (rest of parser methods - would continue with each parse method)
    fn parse_dim(&mut self) -> QResult<Statement> {
        self.advance(); // DIM
        let mut vars = Vec::new();

        loop {
            let shared = if self.check(Token::Shared) {
                self.advance();
                true
            } else {
                false
            };

            let name = self.expect_identifier()?;
            let var_name = name.clone();
            let mut suffix = None;

            // Check for type suffix on identifier
            if let Some(Token::IntegerSuffix) = self.peek_token() {
                suffix = Some(qb_core::data_types::TypeSuffix::Integer);
                self.advance();
            } else if let Some(Token::LongSuffix) = self.peek_token() {
                suffix = Some(qb_core::data_types::TypeSuffix::Long);
                self.advance();
            } else if let Some(Token::SingleSuffix) = self.peek_token() {
                suffix = Some(qb_core::data_types::TypeSuffix::Single);
                self.advance();
            } else if let Some(Token::DoubleSuffix) = self.peek_token() {
                suffix = Some(qb_core::data_types::TypeSuffix::Double);
                self.advance();
            } else if let Some(Token::StringSuffix) = self.peek_token() {
                suffix = Some(qb_core::data_types::TypeSuffix::String);
                self.advance();
            }

            // Check for array bounds
            let bounds = if self.check(Token::LParen) {
                Some(self.parse_dim_bounds()?)
            } else {
                None
            };

            // Check for AS type
            let type_spec = if self.check(Token::As) {
                self.advance();
                Some(self.parse_type_spec()?)
            } else {
                None
            };

            vars.push(DimItem {
                name: qb_core::data_types::VariableId::new(var_name, suffix),
                bounds,
                type_spec,
                shared,
            });

            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(Statement::Dim { vars })
    }

    fn parse_dim_bounds(&mut self) -> QResult<Vec<ArrayBounds>> {
        self.expect(Token::LParen)?;
        let mut bounds = Vec::new();

        loop {
            let lower = if self.peek_token() == Some(&Token::To) {
                0
            } else {
                let expr = self.parse_expression()?;
                match expr {
                    Expression::Integer(n) => n,
                    Expression::Long(n) => n as i32,
                    _ => 0,
                }
            };

            let upper = if self.check(Token::To) {
                self.advance();
                let expr = self.parse_expression()?;
                match expr {
                    Expression::Integer(n) => n,
                    Expression::Long(n) => n as i32,
                    _ => 10,
                }
            } else {
                // If no TO specified, default lower is 0 (or 1 for QBASIC compatibility)
                // DIM arr(5) means arr(0 TO 5) - 6 elements
                lower
            };
            
            // Adjust: if no TO was specified, we need to swap the logic
            // The first number parsed becomes upper bound, and lower bound defaults to 0
            let (actual_lower, actual_upper) = if !self.check(Token::To) && upper == lower {
                // This was a single number like DIM arr(5)
                (0, lower)
            } else {
                (lower, upper)
            };

            bounds.push(ArrayBounds::new(actual_lower, actual_upper));

            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(Token::RParen)?;
        Ok(bounds)
    }

    fn parse_const(&mut self) -> QResult<Statement> {
        self.advance(); // CONST
        let name = self.expect_identifier()?;
        let suffix = self.parse_optional_suffix();
        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;

        Ok(Statement::Const {
            name: qb_core::data_types::VariableId::new(name, suffix),
            value,
        })
    }

    fn parse_deftype(&mut self) -> QResult<Statement> {
        let type_char = match self.peek_token() {
            Some(Token::DefInt) => 'I',
            Some(Token::DefLng) => 'L',
            Some(Token::DefSng) => 'S',
            Some(Token::DefDbl) => 'D',
            Some(Token::DefStr) => '$',
            _ => 'S',
        };
        self.advance();

        let start = self.expect_identifier()?.chars().next().unwrap_or('A');
        let end = if self.check(Token::Minus) {
            self.advance();
            self.expect_identifier()?.chars().next().unwrap_or('Z')
        } else {
            start
        };

        self.declaration_manager.set_default_type(type_char, start, end);

        Ok(Statement::DefType { type_char, letter_range: (start, end) })
    }

    fn parse_type_def(&mut self) -> QResult<Statement> {
        self.advance(); // TYPE
        let name = self.expect_identifier()?;
        self.expect_newline()?;
        let mut fields = Vec::new();

        while !self.check(Token::End) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(Token::End) {
                break;
            }
            let field_name = self.expect_identifier()?;
            self.expect(Token::As)?;
            let type_spec = self.parse_type_spec()?;
            fields.push((field_name, type_spec));
            // Optional newline after each field
            if self.check(Token::NewLine) {
                self.advance();
            }
        }

        self.expect(Token::End)?;
        self.skip_newlines();
        if self.check(Token::Type) {
            self.advance();
        }

        self.declaration_manager.add_user_type(name.clone(), fields.clone());

        Ok(Statement::TypeDef { name, fields })
    }

    fn parse_if(&mut self) -> QResult<Statement> {
        self.advance(); // IF
        let condition = self.parse_expression()?;
        self.expect(Token::Then)?;

        // Check for single-line IF
        let is_single_line = !matches!(self.peek_token(), Some(Token::NewLine) | None);

        let mut then_branch = Vec::new();
        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        if is_single_line {
            // Single line IF
            while !self.check(Token::Else) && !self.is_at_end() {
                if self.check(Token::NewLine) {
                    break;
                }
                then_branch.push(self.parse_statement()?);
            }

            if self.check(Token::Else) {
                self.advance();
                let mut else_stmts = Vec::new();
                while !self.is_at_end() && !self.check(Token::NewLine) {
                    else_stmts.push(self.parse_statement()?);
                }
                else_branch = Some(else_stmts);
            }
        } else {
            // Multi-line IF
            self.expect_newline()?;

            // Parse THEN branch - stop at ELSE, ELSEIF, or END IF (not just END)
            loop {
                self.skip_newlines();
                if self.check(Token::Else) || self.check(Token::ElseIf) || self.is_at_end() {
                    break;
                }
                // Check for END IF
                if self.check(Token::End) && self.peek_next_token() == Some(&Token::If) {
                    break; // This is END IF, stop here
                }
                // Otherwise this is just END (program end), parse it as statement
                let stmt = self.parse_statement()?;
                then_branch.push(stmt);
            }

            // Parse ELSEIF branches
            while self.check(Token::ElseIf) {
                self.advance();
                let elseif_cond = self.parse_expression()?;
                self.expect(Token::Then)?;
                self.expect_newline()?;
                let mut elseif_body = Vec::new();
                loop {
                    self.skip_newlines();
                    if self.check(Token::Else) || self.check(Token::ElseIf) || self.is_at_end() {
                        break;
                    }
                    // Check for END IF
                    if self.check(Token::End) && self.peek_next_token() == Some(&Token::If) {
                        break; // This is END IF, stop here
                    }
                    // Otherwise this is just END, parse it as statement
                    let stmt = self.parse_statement()?;
                    elseif_body.push(stmt);
                }
                else_if_branches.push((elseif_cond, elseif_body));
            }

            // Parse ELSE branch
            if self.check(Token::Else) {
                self.advance();
                self.expect_newline()?;
                let mut else_stmts = Vec::new();
                loop {
                    self.skip_newlines();
                    if self.is_at_end() {
                        break;
                    }
                    // Check for END IF
                    if self.check(Token::End) && self.peek_next_token() == Some(&Token::If) {
                        break; // This is END IF, stop here
                    }
                    // Otherwise this is just END (program end), parse it as statement
                    let stmt = self.parse_statement()?;
                    else_stmts.push(stmt);
                }
                else_branch = Some(else_stmts);
            }

            // Expect END IF
            self.skip_newlines();
            self.expect(Token::End)?;
            self.skip_newlines();
            if self.check(Token::If) {
                self.advance();
            }
        }

        Ok(Statement::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            is_single_line,
        })
    }

    fn parse_for(&mut self) -> QResult<Statement> {
        self.advance(); // FOR
        let var_name = self.expect_identifier()?;
        let suffix = self.parse_optional_suffix();
        let var = qb_core::data_types::VariableId::new(var_name, suffix);

        self.expect(Token::Equal)?;
        let start = self.parse_expression()?;
        self.expect(Token::To)?;
        let end = self.parse_expression()?;

        let step = if self.check(Token::Step) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_newline()?;

        let mut body = Vec::new();
        self.in_loop = true;

        while !self.check(Token::Next) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::Next)?;
        // Optional variable name after NEXT
        if let Some(Token::Identifier(_)) = self.peek_token() {
            self.advance();
        }

        self.in_loop = false;

        Ok(Statement::For { var, start, end, step, body })
    }

    fn parse_while(&mut self) -> QResult<Statement> {
        self.advance(); // WHILE
        let condition = self.parse_expression()?;
        self.expect_newline()?;

        let mut body = Vec::new();
        self.in_loop = true;

        while !self.check(Token::Wend) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::Wend)?;
        self.in_loop = false;

        Ok(Statement::While { condition, body })
    }

    fn parse_do(&mut self) -> QResult<Statement> {
        self.advance(); // DO
        
        // Check for DO WHILE or DO UNTIL
        if self.check(Token::While) {
            self.advance();
            let cond = self.parse_expression()?;
            self.expect_newline()?;
            self.in_loop = true;
            let mut body = Vec::new();
            while !self.check(Token::Loop) && !self.is_at_end() {
                body.push(self.parse_statement()?);
                self.skip_newlines();
            }
            self.expect(Token::Loop)?;
            self.in_loop = false;
            return Ok(Statement::DoWhile { condition: cond, body });
        }
        
        if self.check(Token::Until) {
            self.advance();
            let cond = self.parse_expression()?;
            self.expect_newline()?;
            self.in_loop = true;
            let mut body = Vec::new();
            while !self.check(Token::Loop) && !self.is_at_end() {
                body.push(self.parse_statement()?);
                self.skip_newlines();
            }
            self.expect(Token::Loop)?;
            self.in_loop = false;
            return Ok(Statement::DoUntil { condition: cond, body });
        }
        
        // DO ... LOOP form
        self.expect_newline()?;
        self.in_loop = true;
        let mut body = Vec::new();
        while !self.check(Token::Loop) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::Loop)?;

        // Check for LOOP WHILE/UNTIL
        if self.check(Token::While) {
            self.advance();
            let cond = self.parse_expression()?;
            self.in_loop = false;
            return Ok(Statement::DoLoop { body, condition: Some(cond), is_until: false });
        }
        
        if self.check(Token::Until) {
            self.advance();
            let cond = self.parse_expression()?;
            self.in_loop = false;
            return Ok(Statement::DoLoop { body, condition: Some(cond), is_until: true });
        }

        self.in_loop = false;
        Ok(Statement::DoLoop { body, condition: None, is_until: false })
    }

    fn parse_print(&mut self) -> QResult<Statement> {
        self.advance(); // PRINT

        let mut items = Vec::new();

        while !self.check(Token::NewLine) && !self.is_at_end() {
            if self.check(Token::Semicolon) {
                self.advance();
                items.push(PrintItem::Semicolon);
            } else if self.check(Token::Comma) {
                self.advance();
                items.push(PrintItem::Comma);
            } else {
                items.push(PrintItem::Expression(self.parse_expression()?));
            }
        }

        Ok(Statement::Print { items, is_question: false })
    }

    fn parse_input(&mut self) -> QResult<Statement> {
        self.advance(); // INPUT
        let prompt = if let Some(Token::String(s)) = self.peek_token() {
            let s = s.clone();
            self.advance();
            if self.check(Token::Semicolon) || self.check(Token::Comma) {
                self.advance();
            }
            Some(s)
        } else {
            None
        };

        let mut vars = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            let suffix = self.parse_optional_suffix();
            vars.push(qb_core::data_types::VariableId::new(name, suffix));

            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(Statement::Input { prompt, vars })
    }

    fn parse_goto(&mut self) -> QResult<Statement> {
        self.advance(); // GOTO
        let label = self.expect_identifier()?;
        Ok(Statement::Goto { label })
    }

    fn parse_gosub(&mut self) -> QResult<Statement> {
        self.advance(); // GOSUB
        let label = self.expect_identifier()?;
        Ok(Statement::Gosub { label })
    }

    fn parse_expression(&mut self) -> QResult<Expression> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> QResult<Expression> {
        let mut left = self.parse_and()?;
        while self.check(Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> QResult<Expression> {
        let mut left = self.parse_equality()?;
        while self.check(Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> QResult<Expression> {
        let mut left = self.parse_comparison()?;
        while let Some(op) = self.match_equality_op() {
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> QResult<Expression> {
        let mut left = self.parse_addition()?;
        while let Some(op) = self.match_comparison_op() {
            let right = self.parse_addition()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> QResult<Expression> {
        let mut left = self.parse_multiplication()?;
        while self.check(Token::Plus) || self.check(Token::Minus) {
            let op = if self.check(Token::Plus) { BinaryOp::Add } else { BinaryOp::Subtract };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> QResult<Expression> {
        let mut left = self.parse_power()?;
        while self.check(Token::Multiply) || self.check(Token::Divide) || self.check(Token::IntDivide) || self.check(Token::Modulo) {
            let op = if self.check(Token::Multiply) {
                BinaryOp::Multiply
            } else if self.check(Token::Divide) {
                BinaryOp::Divide
            } else if self.check(Token::IntDivide) {
                BinaryOp::IntDivide
            } else {
                BinaryOp::Modulo
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> QResult<Expression> {
        let left = self.parse_unary()?;
        if self.check(Token::Power) {
            self.advance();
            let right = self.parse_power()?; // Right-associative
            Ok(Expression::Binary {
                op: BinaryOp::Power,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> QResult<Expression> {
        if self.check(Token::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expression::Negate(Box::new(expr)))
        } else if self.check(Token::Plus) {
            self.advance();
            self.parse_unary()
        } else if self.check(Token::Not) {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expression::Not(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> QResult<Expression> {
        if let Some(token) = self.peek_token() {
            if let Some(name) = token.as_builtin_function_name() {
                let name = name.to_string();
                self.advance();
                
                let args = if self.check(Token::LParen) {
                    self.parse_argument_list()?
                } else {
                    Vec::new()
                };
                return Ok(Expression::FunctionCall { name, args });
            }
        }

        match self.peek_token() {
            Some(Token::Integer(n)) => {
                let val = *n;
                self.advance();
                Ok(Expression::Integer(val))
            }
            Some(Token::Long(n)) => {
                let val = *n;
                self.advance();
                Ok(Expression::Long(val))
            }
            Some(Token::Single(n)) => {
                let val = *n;
                self.advance();
                Ok(Expression::Single(val))
            }
            Some(Token::Double(n)) => {
                let val = *n;
                self.advance();
                Ok(Expression::Double(val))
            }
            Some(Token::String(s)) => {
                let val = s.clone();
                self.advance();
                Ok(Expression::String(val))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                
                if self.check(Token::LParen) {
                    // Function call or array access
                    let args = self.parse_argument_list()?;
                    // Check if it's a known function
                    if self.is_builtin_function(&name) {
                        Ok(Expression::FunctionCall { name, args })
                    } else {
                        Ok(Expression::ArrayAccess(
                            qb_core::data_types::VariableId::new(name, None),
                            args
                        ))
                    }
                } else {
                    Ok(Expression::Variable(qb_core::data_types::VariableId::new(name, None)))
                }
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => {
                let (line, col) = self.current_pos();
                Err(QError::compile("Expected expression", line, col))
            }
        }
    }

    fn parse_argument_list(&mut self) -> QResult<Vec<Expression>> {
        self.expect(Token::LParen)?;
        let mut args = Vec::new();

        if !self.check(Token::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if self.check(Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RParen)?;
        Ok(args)
    }

    fn parse_array_indices(&mut self) -> QResult<Vec<Expression>> {
        self.expect(Token::LParen)?;
        let mut indices = Vec::new();

        loop {
            indices.push(self.parse_expression()?);
            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(Token::RParen)?;
        Ok(indices)
    }

    fn parse_type_spec(&mut self) -> QResult<TypeSpec> {
        match self.peek_token() {
            Some(Token::IntegerType) => {
                self.advance();
                Ok(TypeSpec::Simple("INTEGER".to_string()))
            }
            Some(Token::LongType) => {
                self.advance();
                Ok(TypeSpec::Simple("LONG".to_string()))
            }
            Some(Token::SingleType) => {
                self.advance();
                Ok(TypeSpec::Simple("SINGLE".to_string()))
            }
            Some(Token::DoubleType) => {
                self.advance();
                Ok(TypeSpec::Simple("DOUBLE".to_string()))
            }
            Some(Token::StringType) => {
                self.advance();
                if self.check(Token::Multiply) {
                    self.advance();
                    let len = self.parse_expression()?;
                    Ok(TypeSpec::FixedString(len))
                } else {
                    Ok(TypeSpec::Simple("STRING".to_string()))
                }
            }
            // QB64 extended types
            Some(Token::Integer64Type) => {
                self.advance();
                Ok(TypeSpec::Simple("_INTEGER64".to_string()))
            }
            Some(Token::UnsignedIntegerType) => {
                self.advance();
                Ok(TypeSpec::Simple("_UNSIGNED INTEGER".to_string()))
            }
            Some(Token::UnsignedLongType) => {
                self.advance();
                Ok(TypeSpec::Simple("_UNSIGNED LONG".to_string()))
            }
            Some(Token::UnsignedInteger64Type) => {
                self.advance();
                Ok(TypeSpec::Simple("_UNSIGNED _INTEGER64".to_string()))
            }
            Some(Token::FloatType) => {
                self.advance();
                Ok(TypeSpec::Simple("_FLOAT".to_string()))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(TypeSpec::UserDefined(name))
            }
            _ => {
                let (line, col) = self.current_pos();
                Err(QError::compile("Expected type specification", line, col))
            }
        }
    }

    fn parse_optional_suffix(&mut self) -> Option<qb_core::data_types::TypeSuffix> {
        match self.peek_token() {
            Some(Token::IntegerSuffix) => { self.advance(); Some(qb_core::data_types::TypeSuffix::Integer) }
            Some(Token::LongSuffix) => { self.advance(); Some(qb_core::data_types::TypeSuffix::Long) }
            Some(Token::SingleSuffix) => { self.advance(); Some(qb_core::data_types::TypeSuffix::Single) }
            Some(Token::DoubleSuffix) => { self.advance(); Some(qb_core::data_types::TypeSuffix::Double) }
            Some(Token::StringSuffix) => { self.advance(); Some(qb_core::data_types::TypeSuffix::String) }
            _ => None,
        }
    }

    // Stub methods for statements not fully implemented
    fn parse_select(&mut self) -> QResult<Statement> {
        self.advance(); // SELECT
        self.expect(Token::Case)?;
        let expr = self.parse_expression()?;
        self.expect_newline()?;
        
        let mut cases = Vec::new();
        let mut case_else = None;
        
        // Parse CASE clauses
        while !self.check(Token::End) && !self.is_at_end() {
            self.skip_newlines();
            
            if self.check(Token::End) {
                break;
            }
            
            if self.check(Token::Case) {
                self.advance(); // CASE
                
                // Check for CASE ELSE
                if self.check(Token::Else) {
                    self.advance(); // ELSE
                    self.expect_newline()?;
                    let mut else_stmts = Vec::new();
                    while !self.check(Token::End) && !self.check(Token::Case) && !self.is_at_end() {
                        self.skip_newlines();
                        if self.check(Token::End) || self.check(Token::Case) {
                            break;
                        }
                        let stmt = self.parse_statement()?;
                        else_stmts.push(stmt);
                    }
                    case_else = Some(else_stmts);
                } else {
                    // Parse case conditions
                    let mut conditions = Vec::new();
                    
                    loop {
                        // Check for IS keyword
                        if self.check(Token::Is) {
                            self.advance(); // IS
                            let op = if let Some(token) = self.peek_token() {
                                token.clone()
                            } else {
                                break;
                            };
                            self.advance();
                            let expr2 = self.parse_expression()?;
                            conditions.push(CaseCondition::Is(op, expr2));
                        } 
                        // Check for range (e.g., 1 TO 10)
                        else {
                            let expr1 = self.parse_expression()?;
                            if self.check(Token::To) {
                                self.advance(); // TO
                                let expr2 = self.parse_expression()?;
                                conditions.push(CaseCondition::Range(expr1, expr2));
                            } else {
                                conditions.push(CaseCondition::Expression(expr1));
                            }
                        }
                        
                        if self.check(Token::Comma) {
                            self.advance(); // Comma for multiple conditions
                        } else {
                            break;
                        }
                    }
                    
                    self.expect_newline()?;
                    
                    // Parse case body
                    let mut body = Vec::new();
                    while !self.check(Token::End) && !self.check(Token::Case) && !self.is_at_end() {
                        self.skip_newlines();
                        if self.check(Token::End) || self.check(Token::Case) {
                            break;
                        }
                        let stmt = self.parse_statement()?;
                        body.push(stmt);
                    }
                    
                    cases.push(CaseClause { conditions, body });
                }
            } else {
                // Unexpected token, skip
                self.advance();
            }
        }
        
        self.expect(Token::End)?;
        self.skip_newlines();
        if self.check(Token::Select) {
            self.advance();
        }
        
        Ok(Statement::Select { expr, cases, case_else })
    }

    fn parse_on(&mut self) -> QResult<Statement> {
        self.advance(); // ON
        let _expr = self.parse_expression()?;
        // Simplified - just consume tokens
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("ON GOTO/GOSUB")))
    }

    fn parse_sub(&mut self) -> QResult<Statement> {
        self.advance(); // SUB
        let name = self.expect_identifier()?;
        let params = if self.check(Token::LParen) {
            self.parse_param_list()?
        } else {
            Vec::new()
        };
        self.expect_newline()?;
        
        self.in_sub = true;
        let mut body = Vec::new();
        while !self.check(Token::End) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }
        self.expect(Token::End)?;
        if self.check(Token::Sub) {
            self.advance();
        }
        self.in_sub = false;
        
        Ok(Statement::Sub { name, params, body, is_static: false })
    }

    fn parse_function(&mut self) -> QResult<Statement> {
        self.advance(); // FUNCTION
        let name = self.expect_identifier()?;
        let params = if self.check(Token::LParen) {
            self.parse_param_list()?
        } else {
            Vec::new()
        };
        
        let return_type = if self.check(Token::As) {
            self.advance();
            Some(self.parse_type_spec()?)
        } else {
            None
        };
        
        self.expect_newline()?;
        
        self.in_function = true;
        let mut body = Vec::new();
        while !self.check(Token::End) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }
        self.expect(Token::End)?;
        if self.check(Token::Function) {
            self.advance();
        }
        self.in_function = false;
        
        Ok(Statement::Function { name, params, return_type, body, is_static: false })
    }

    fn parse_param_list(&mut self) -> QResult<Vec<ParamType>> {
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        
        if !self.check(Token::RParen) {
            loop {
                // Check for BYVAL/BYREF
                let mut by_val = false;
                if let Some(Token::Identifier(s)) = self.peek_token() {
                    let upper = s.to_uppercase();
                    if upper == "BYVAL" {
                        by_val = true;
                        self.advance();
                    } else if upper == "BYREF" {
                        self.advance();
                    }
                }
                
                let name = self.expect_identifier()?;
                let suffix = self.parse_optional_suffix();
                let var = qb_core::data_types::VariableId::new(name, suffix);
                
                if by_val {
                    params.push(ParamType::ByVal(var));
                } else {
                    params.push(ParamType::ByRef(var));
                }
                
                if self.check(Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.expect(Token::RParen)?;
        Ok(params)
    }

    fn parse_declare(&mut self) -> QResult<Statement> {
        self.advance(); // DECLARE
        let is_sub = self.check(Token::Sub);
        if is_sub {
            self.advance();
        } else {
            self.expect(Token::Function)?;
        }
        let name = self.expect_identifier()?;
        let params = if self.check(Token::LParen) {
            self.parse_param_list()?
        } else {
            Vec::new()
        };
        Ok(Statement::Declare { is_sub, name, params })
    }

    fn parse_call(&mut self) -> QResult<Statement> {
        self.advance(); // CALL
        let name = self.expect_identifier()?;
        let args = if self.check(Token::LParen) {
            self.parse_argument_list()?
        } else {
            Vec::new()
        };
        Ok(Statement::Call { name, args: args.into_iter().map(Argument::ByVal).collect() })
    }

    fn parse_exit(&mut self) -> QResult<Statement> {
        self.advance(); // EXIT
        match self.peek_token() {
            Some(Token::Sub) => { self.advance(); Ok(Statement::ExitSub) }
            Some(Token::Function) => { self.advance(); Ok(Statement::ExitFunction) }
            Some(Token::For) => { self.advance(); Ok(Statement::ExitFor) }
            Some(Token::Do) => { self.advance(); Ok(Statement::ExitDo) }
            _ => {
                let (line, col) = self.current_pos();
                Err(QError::compile("Expected SUB, FUNCTION, FOR, or DO after EXIT", line, col))
            }
        }
    }

    fn parse_line_input(&mut self) -> QResult<Statement> {
        self.advance(); // LINE INPUT
        let prompt = if let Some(Token::String(s)) = self.peek_token() {
            let s = s.clone();
            self.advance();
            if self.check(Token::Semicolon) || self.check(Token::Comma) {
                self.advance();
            }
            Some(s)
        } else {
            None
        };
        let name = self.expect_identifier()?;
        let suffix = self.parse_optional_suffix();
        let var = qb_core::data_types::VariableId::new(name, suffix);
        Ok(Statement::LineInput { prompt, var })
    }

    fn parse_write(&mut self) -> QResult<Statement> {
        self.advance(); // WRITE
        let mut items = Vec::new();
        while !self.check(Token::NewLine) && !self.is_at_end() {
            items.push(self.parse_expression()?);
            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Statement::Write { items })
    }

    fn parse_open(&mut self) -> QResult<Statement> {
        self.advance(); // OPEN
        let filename = self.parse_expression()?;
        
        // Parse FOR mode
        let mode = if self.check(Token::For) {
            self.advance();
            match self.peek_token() {
                Some(Token::Input) => { self.advance(); FileMode::Input }
                Some(Token::Output) => { self.advance(); FileMode::Output }
                Some(Token::Append) => { self.advance(); FileMode::Append }
                Some(Token::Random) => { self.advance(); FileMode::Random }
                Some(Token::Binary) => { self.advance(); FileMode::Binary }
                _ => FileMode::Random,
            }
        } else {
            FileMode::Random
        };
        
        // Parse AS #fileno
        let fileno = if self.check(Token::As) {
            self.advance();
            if self.check(Token::Hash) {
                self.advance();
            }
            self.parse_expression()?
        } else {
            Expression::Integer(1)
        };
        
        Ok(Statement::Open { filename, mode, fileno, reclen: None })
    }

    fn parse_close(&mut self) -> QResult<Statement> {
        self.advance(); // CLOSE
        let fileno = if self.check(Token::Hash) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::Close { fileno })
    }

    fn parse_get(&mut self) -> QResult<Statement> {
        self.advance(); // GET
        // Simplified
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Get { fileno: Expression::Integer(1), record: None, var: qb_core::data_types::VariableId::new("X", None) })
    }

    fn parse_put(&mut self) -> QResult<Statement> {
        self.advance(); // PUT
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Put { fileno: Expression::Integer(1), record: None, var: qb_core::data_types::VariableId::new("X", None) })
    }

    fn parse_seek(&mut self) -> QResult<Statement> {
        self.advance(); // SEEK
        // Simplified
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Seek { fileno: Expression::Integer(1), position: Expression::Integer(1) })
    }

    fn parse_lock(&mut self) -> QResult<Statement> {
        self.advance(); // LOCK
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Lock { fileno: Expression::Integer(1), record: None })
    }

    fn parse_unlock(&mut self) -> QResult<Statement> {
        self.advance(); // UNLOCK
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Unlock { fileno: Expression::Integer(1), record: None })
    }

    fn parse_print_hash(&mut self) -> QResult<Statement> {
        self.advance(); // PRINT #
        let fileno = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let mut items = Vec::new();
        while !self.check(Token::NewLine) && !self.is_at_end() {
            if self.check(Token::Semicolon) {
                self.advance();
                items.push(PrintItem::Semicolon);
            } else if self.check(Token::Comma) {
                self.advance();
                items.push(PrintItem::Comma);
            } else {
                items.push(PrintItem::Expression(self.parse_expression()?));
            }
        }
        Ok(Statement::PrintHash { fileno, items })
    }

    fn parse_input_hash(&mut self) -> QResult<Statement> {
        self.advance(); // INPUT #
        let fileno = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let mut vars = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            let suffix = self.parse_optional_suffix();
            vars.push(qb_core::data_types::VariableId::new(name, suffix));
            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Statement::InputHash { fileno, vars })
    }

    fn parse_screen(&mut self) -> QResult<Statement> {
        self.advance(); // SCREEN
        let mode = self.parse_expression()?;
        Ok(Statement::Screen { mode })
    }

    fn parse_pset(&mut self) -> QResult<Statement> {
        self.advance(); // PSET
        let x = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let y = self.parse_expression()?;
        let color = if self.check(Token::Comma) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::PSet { x, y, color })
    }

    fn parse_preset(&mut self) -> QResult<Statement> {
        self.advance(); // PRESET
        let x = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let y = self.parse_expression()?;
        Ok(Statement::PReset { x, y })
    }

    fn parse_line(&mut self) -> QResult<Statement> {
        self.advance(); // LINE
        // Simplified
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("LINE")))
    }

    fn parse_circle(&mut self) -> QResult<Statement> {
        self.advance(); // CIRCLE
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("CIRCLE")))
    }

    fn parse_draw(&mut self) -> QResult<Statement> {
        self.advance(); // DRAW
        let command = self.parse_expression()?;
        Ok(Statement::Draw { command })
    }

    fn parse_paint(&mut self) -> QResult<Statement> {
        self.advance(); // PAINT
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("PAINT")))
    }

    fn parse_view(&mut self) -> QResult<Statement> {
        self.advance(); // VIEW
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("VIEW")))
    }

    fn parse_window(&mut self) -> QResult<Statement> {
        self.advance(); // WINDOW
        while !self.check(Token::NewLine) && !self.is_at_end() {
            self.advance();
        }
        Ok(Statement::Rem(String::from("WINDOW")))
    }

    fn parse_palette(&mut self) -> QResult<Statement> {
        self.advance(); // PALETTE
        Ok(Statement::Palette { attribute: None, color: None })
    }

    fn parse_color(&mut self) -> QResult<Statement> {
        self.advance(); // COLOR
        let foreground = if !self.check(Token::NewLine) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        let background = if self.check(Token::Comma) {
            self.advance();
            if self.check(Token::Comma) {
                None
            } else {
                Some(self.parse_expression()?)
            }
        } else {
            None
        };
        let border = if self.check(Token::Comma) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::Color { foreground, background, border })
    }

    fn parse_locate(&mut self) -> QResult<Statement> {
        self.advance(); // LOCATE
        let row = if !self.check(Token::NewLine) && !self.check(Token::Comma) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        let col = if self.check(Token::Comma) {
            self.advance();
            if self.check(Token::Comma) {
                None
            } else {
                Some(self.parse_expression()?)
            }
        } else {
            None
        };
        Ok(Statement::Locate { row, col, cursor: None, start: None, stop: None })
    }

    fn parse_width(&mut self) -> QResult<Statement> {
        self.advance(); // WIDTH
        let value = self.parse_expression()?;
        Ok(Statement::Width { value })
    }

    fn parse_sound(&mut self) -> QResult<Statement> {
        self.advance(); // SOUND
        let frequency = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let duration = self.parse_expression()?;
        Ok(Statement::Sound { frequency, duration })
    }

    fn parse_play(&mut self) -> QResult<Statement> {
        self.advance(); // PLAY
        let command = self.parse_expression()?;
        Ok(Statement::Play { command })
    }

    fn parse_poke(&mut self) -> QResult<Statement> {
        self.advance(); // POKE
        let address = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let value = self.parse_expression()?;
        Ok(Statement::Poke { address, value })
    }

    fn parse_defseg(&mut self) -> QResult<Statement> {
        self.advance(); // DEF SEG
        let segment = if !self.check(Token::NewLine) {
            self.expect(Token::Equal)?;
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::DefSeg { segment })
    }

    fn parse_data(&mut self) -> QResult<Statement> {
        self.advance(); // DATA
        let mut values = Vec::new();
        
        // DATA statement comes as a single string from lexer, need to parse it manually
        let data_str = if let Some(Token::String(s)) = self.peek_token() {
            let s = s.clone();
            self.advance();
            Some(s)
        } else {
            None
        };
        
        if let Some(s) = data_str {
            // Split by comma and parse each value
            for part in s.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    values.push(self.parse_data_value(trimmed)?);
                }
            }
        }
        
        Ok(Statement::Data { values })
    }
    
    fn parse_data_value(&self, s: &str) -> QResult<Expression> {
        // Try to parse as integer
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Expression::Integer(n));
        }
        // Try to parse as float
        if let Ok(n) = s.parse::<f64>() {
            return Ok(Expression::Double(n));
        }
        // Try to parse as string (quoted)
        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            return Ok(Expression::String(s[1..s.len()-1].to_string()));
        }
        // Default to string
        Ok(Expression::String(s.to_string()))
    }

    fn parse_read(&mut self) -> QResult<Statement> {
        self.advance(); // READ
        let mut vars = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            let suffix = self.parse_optional_suffix();
            vars.push(qb_core::data_types::VariableId::new(name, suffix));
            if self.check(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Statement::Read { vars })
    }

    fn parse_restore(&mut self) -> QResult<Statement> {
        self.advance(); // RESTORE
        let label = if !self.check(Token::NewLine) {
            Some(self.expect_identifier()?)
        } else {
            None
        };
        Ok(Statement::Restore { label })
    }

    fn parse_environ(&mut self) -> QResult<Statement> {
        self.advance(); // ENVIRON
        let expr = self.parse_expression()?;
        Ok(Statement::Environ { expr })
    }

    fn parse_shell(&mut self) -> QResult<Statement> {
        self.advance(); // SHELL
        let command = if !self.check(Token::NewLine) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::Shell { command })
    }

    fn parse_on_error(&mut self) -> QResult<Statement> {
        self.advance(); // ON
        self.expect(Token::Error)?;
        self.expect(Token::GoTo)?;
        let label = self.expect_identifier()?;
        Ok(Statement::OnError { label })
    }

    fn parse_resume(&mut self) -> QResult<Statement> {
        self.advance(); // RESUME
        if self.check(Token::Next) {
            self.advance();
            Ok(Statement::Resume { next: true, label: None })
        } else if !self.check(Token::NewLine) {
            let label = self.expect_identifier()?;
            Ok(Statement::Resume { next: false, label: Some(label) })
        } else {
            Ok(Statement::Resume { next: false, label: None })
        }
    }

    fn parse_error(&mut self) -> QResult<Statement> {
        self.advance(); // ERROR
        let code = self.parse_expression()?;
        Ok(Statement::Error { code })
    }

    fn parse_randomize(&mut self) -> QResult<Statement> {
        self.advance(); // RANDOMIZE
        // Parse optional seed expression (e.g., TIMER or a number)
        if !self.check(Token::NewLine) && !self.is_at_end() {
            let _seed = self.parse_expression()?;
        }
        // Return as Rem for now (RANDOMIZE doesn't need AST representation yet)
        Ok(Statement::Rem(String::from("RANDOMIZE")))
    }

    // Helper methods
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current).map(|t| &t.token)
    }

    fn peek_next_token(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1).map(|t| &t.token)
    }

    fn advance(&mut self) -> &TokenInfo {
        let token = &self.tokens[self.current];
        if !self.is_at_end() {
            self.current += 1;
        }
        token
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek_token(), Some(Token::EOF))
    }

    fn check(&self, token: Token) -> bool {
        self.peek_token() == Some(&token)
    }

    fn expect(&mut self, expected: Token) -> QResult<()> {
        if self.check(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            let (line, col) = self.current_pos();
            Err(QError::compile(
                format!("Expected {:?}, found {:?}", expected, self.peek_token()),
                line,
                col
            ))
        }
    }

    fn expect_identifier(&mut self) -> QResult<String> {
        match self.peek_token() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => {
                let (line, col) = self.current_pos();
                Err(QError::compile("Expected identifier", line, col))
            }
        }
    }

    fn expect_newline(&mut self) -> QResult<()> {
        if self.check(Token::NewLine) {
            self.advance();
            Ok(())
        } else {
            let (line, col) = self.current_pos();
            Err(QError::compile("Expected newline", line, col))
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(Token::NewLine) {
            self.advance();
        }
    }

    fn current_pos(&self) -> (usize, usize) {
        if let Some(token) = self.tokens.get(self.current) {
            (token.line, token.column)
        } else {
            (0, 0)
        }
    }

    fn current_line(&self) -> usize {
        self.current_pos().0
    }

    fn current_column(&self) -> usize {
        self.current_pos().1
    }

    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.check(Token::Equal) {
            self.advance();
            Some(BinaryOp::Equal)
        } else if self.check(Token::NotEqual) {
            self.advance();
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }

    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.check(Token::Less) {
            self.advance();
            Some(BinaryOp::Less)
        } else if self.check(Token::LessEqual) {
            self.advance();
            Some(BinaryOp::LessEqual)
        } else if self.check(Token::Greater) {
            self.advance();
            Some(BinaryOp::Greater)
        } else if self.check(Token::GreaterEqual) {
            self.advance();
            Some(BinaryOp::GreaterEqual)
        } else {
            None
        }
    }

    fn is_builtin_function(&self, name: &str) -> bool {
        let upper = name.to_uppercase();
        matches!(upper.as_str(),
            "ABS" | "ASC" | "ATN" | "CHR$" | "COS" | "DATE$" | "EXP" | "FIX" | "INT" |
            "INSTR" | "LCASE$" | "LEFT$" | "LEN" | "LOG" | "MID$" | "RIGHT$" | "RND" |
            "SGN" | "SIN" | "SPACE$" | "SQR" | "STR$" | "STRING$" | "TAN" | "TIME$" |
            "TIMER" | "UCASE$" | "VAL" | "CINT" | "CLNG" | "CSNG" | "CDBL" | "CSTR" |
            "PEEK" | "INP" | "EOF" | "LOF" | "LOC" | "FREEFILE" | "LBOUND" | "UBOUND"
        )
    }
}

/// Parse source code into an AST
pub fn parse(tokens: Vec<TokenInfo>) -> QResult<Program> {
    let parser = Parser::new(tokens);
    parser.parse()
}
