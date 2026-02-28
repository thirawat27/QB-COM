use crate::tokens::{Token, TokenInfo, string_to_keyword};
use qb_core::errors::{QError, QResult};

/// Character stream for lexical analysis
pub struct CharStream {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl CharStream {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.source.get(self.position) {
            let ch = *c;
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn substring(&self, start: usize, end: usize) -> String {
        self.source[start..end].iter().collect()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn skip_line(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.advance();
                break;
            }
            self.advance();
        }
    }
}

/// QBasic Tokenizer/Scanner
pub struct Scanner {
    stream: CharStream,
    tokens: Vec<TokenInfo>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            stream: CharStream::new(source),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> QResult<Vec<TokenInfo>> {
        while !self.stream.is_at_end() {
            self.scan_token()?;
        }
        
        // Add EOF token
        let line = self.stream.line();
        let col = self.stream.column();
        self.tokens.push(TokenInfo::new(Token::EOF, line, col, 0));
        
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> QResult<()> {
        let start_line = self.stream.line();
        let start_col = self.stream.column();
        
        self.stream.skip_whitespace();
        
        if self.stream.is_at_end() {
            return Ok(());
        }

        let _start_pos = self.stream.position();
        let c = self.stream.peek().unwrap();

        match c {
            // Comments
            '\'' => {
                self.stream.skip_line();
                // Skip newline if present
                if self.stream.peek() == Some('\n') {
                    self.add_token(Token::NewLine, start_line, start_col, 1);
                    self.stream.advance();
                }
            }
            
            // Newlines
            '\n' => {
                self.add_token(Token::NewLine, start_line, start_col, 1);
                self.stream.advance();
            }
            
            // String literals
            '"' => self.scan_string(start_line, start_col)?,
            
            // Numbers
            '0'..='9' | '.' => {
                if c == '.' && !matches!(self.stream.peek_next(), Some('0'..='9')) {
                    // Not a number, just a period
                    self.stream.advance();
                    self.add_token(Token::Period, start_line, start_col, 1);
                } else {
                    self.scan_number(start_line, start_col)?;
                }
            }
            
            // Hex/Octal literals
            '&' => self.scan_hex_octal(start_line, start_col)?,
            
            // Identifiers and keywords
            'A'..='Z' | 'a'..='z' | '_' => {
                self.scan_identifier(start_line, start_col)?;
            }
            
            // QB64 Metacommands (start with $)
            '$' => {
                self.scan_metacommand(start_line, start_col)?;
            }
            
            // Operators and delimiters
            '+' => {
                self.stream.advance();
                self.add_token(Token::Plus, start_line, start_col, 1);
            }
            '-' => {
                self.stream.advance();
                self.add_token(Token::Minus, start_line, start_col, 1);
            }
            '*' => {
                self.stream.advance();
                self.add_token(Token::Multiply, start_line, start_col, 1);
            }
            '/' => {
                self.stream.advance();
                self.add_token(Token::Divide, start_line, start_col, 1);
            }
            '\\' => {
                self.stream.advance();
                self.add_token(Token::IntDivide, start_line, start_col, 1);
            }
            '^' => {
                self.stream.advance();
                self.add_token(Token::Power, start_line, start_col, 1);
            }
            '(' => {
                self.stream.advance();
                self.add_token(Token::LParen, start_line, start_col, 1);
            }
            ')' => {
                self.stream.advance();
                self.add_token(Token::RParen, start_line, start_col, 1);
            }
            '[' => {
                self.stream.advance();
                self.add_token(Token::LBracket, start_line, start_col, 1);
            }
            ']' => {
                self.stream.advance();
                self.add_token(Token::RBracket, start_line, start_col, 1);
            }
            ',' => {
                self.stream.advance();
                self.add_token(Token::Comma, start_line, start_col, 1);
            }
            ';' => {
                self.stream.advance();
                self.add_token(Token::Semicolon, start_line, start_col, 1);
            }
            ':' => {
                self.stream.advance();
                self.add_token(Token::Colon, start_line, start_col, 1);
            }
            '%' => {
                self.stream.advance();
                self.add_token(Token::IntegerSuffix, start_line, start_col, 1);
            }

            '!' => {
                self.stream.advance();
                self.add_token(Token::SingleSuffix, start_line, start_col, 1);
            }
            '#' => {
                self.stream.advance();
                self.add_token(Token::Hash, start_line, start_col, 1);
            }
            '@' => {
                // Skip @ prefix (used in some QBasic variants)
                self.stream.advance();
            }
            
            // Comparison operators
            '=' => {
                self.stream.advance();
                self.add_token(Token::Equal, start_line, start_col, 1);
            }
            '<' => {
                self.stream.advance();
                match self.stream.peek() {
                    Some('=') => {
                        self.stream.advance();
                        self.add_token(Token::LessEqual, start_line, start_col, 2);
                    }
                    Some('>') => {
                        self.stream.advance();
                        self.add_token(Token::NotEqual, start_line, start_col, 2);
                    }
                    _ => {
                        self.add_token(Token::Less, start_line, start_col, 1);
                    }
                }
            }
            '>' => {
                self.stream.advance();
                match self.stream.peek() {
                    Some('=') => {
                        self.stream.advance();
                        self.add_token(Token::GreaterEqual, start_line, start_col, 2);
                    }
                    Some('<') => {
                        self.stream.advance();
                        self.add_token(Token::NotEqual, start_line, start_col, 2);
                    }
                    _ => {
                        self.add_token(Token::Greater, start_line, start_col, 1);
                    }
                }
            }
            

            
            // Unknown character
            _ => {
                return Err(QError::compile(
                    format!("Unexpected character: {}", c),
                    start_line,
                    start_col
                ));
            }
        }

        Ok(())
    }

    fn scan_string(&mut self, line: usize, col: usize) -> QResult<()> {
        let start_pos = self.stream.position();
        self.stream.advance(); // Skip opening quote

        let mut value = String::new();
        while let Some(c) = self.stream.peek() {
            if c == '"' {
                // Check for escaped quote ("")
                self.stream.advance();
                if self.stream.peek() == Some('"') {
                    value.push('"');
                    self.stream.advance();
                } else {
                    // End of string
                    break;
                }
            } else if c == '\n' {
                return Err(QError::compile(
                    "Unterminated string literal",
                    self.stream.line(),
                    self.stream.column()
                ));
            } else {
                value.push(c);
                self.stream.advance();
            }
        }

        let length = self.stream.position() - start_pos;
        self.add_token(Token::String(value), line, col, length);
        Ok(())
    }

    fn scan_number(&mut self, line: usize, col: usize) -> QResult<()> {
        let start_pos = self.stream.position();
        let mut has_decimal = false;
        let mut has_exponent = false;
        let mut is_double = false;

        // Integer part
        while let Some(c) = self.stream.peek() {
            if c.is_ascii_digit() {
                self.stream.advance();
            } else if c == '.' && !has_decimal && !has_exponent {
                has_decimal = true;
                self.stream.advance();
            } else if (c == 'E' || c == 'e' || c == 'D' || c == 'd') && !has_exponent {
                has_exponent = true;
                if c == 'D' || c == 'd' {
                    is_double = true;
                }
                self.stream.advance();
                // Optional sign
                if let Some(sign) = self.stream.peek() {
                    if sign == '+' || sign == '-' {
                        self.stream.advance();
                    }
                }
            } else {
                break;
            }
        }

        // Check for type suffix
        if let Some(c) = self.stream.peek() {
            match c {
                '%' => {
                    self.stream.advance();
                    let num_str: String = self.stream.source[start_pos..self.stream.position() - 1]
                        .iter().collect();
                    let value: i16 = num_str.parse().map_err(|_| {
                        QError::compile("Invalid integer literal", line, col)
                    })?;
                    self.add_token(Token::Integer(value as i32), line, col, 
                        self.stream.position() - start_pos);
                    return Ok(());
                }
                '&' => {
                    self.stream.advance();
                    // Check for && (Integer64)
                    if self.stream.peek() == Some('&') {
                        self.stream.advance();
                        let num_str: String = self.stream.source[start_pos..self.stream.position() - 2]
                            .iter().collect();
                        let value: i64 = num_str.parse().map_err(|_| {
                            QError::compile("Invalid integer64 literal", line, col)
                        })?;
                        self.add_token(Token::Long(value), line, col, 
                            self.stream.position() - start_pos);
                        return Ok(());
                    }
                    let num_str: String = self.stream.source[start_pos..self.stream.position() - 1]
                        .iter().collect();
                    let value: i64 = num_str.parse().map_err(|_| {
                        QError::compile("Invalid long literal", line, col)
                    })?;
                    self.add_token(Token::Long(value), line, col, 
                        self.stream.position() - start_pos);
                    return Ok(());
                }
                '!' => {
                    self.stream.advance();
                    let num_str: String = self.stream.source[start_pos..self.stream.position() - 1]
                        .iter().collect();
                    let value: f32 = num_str.parse().map_err(|_| {
                        QError::compile("Invalid single literal", line, col)
                    })?;
                    self.add_token(Token::Single(value), line, col, 
                        self.stream.position() - start_pos);
                    return Ok(());
                }
                '#' => {
                    self.stream.advance();
                    let num_str: String = self.stream.source[start_pos..self.stream.position() - 1]
                        .iter().collect();
                    let value: f64 = num_str.parse().map_err(|_| {
                        QError::compile("Invalid double literal", line, col)
                    })?;
                    self.add_token(Token::Double(value), line, col, 
                        self.stream.position() - start_pos);
                    return Ok(());
                }
                _ => {}
            }
        }

        let num_str: String = self.stream.source[start_pos..self.stream.position()]
            .iter().collect();

        // Determine type based on format
        if has_decimal || has_exponent || is_double {
            let value: f64 = num_str.parse().map_err(|_| {
                QError::compile("Invalid floating point literal", line, col)
            })?;
            self.add_token(Token::Double(value), line, col, 
                self.stream.position() - start_pos);
        } else {
            // Try as integer first
            if let Ok(val) = num_str.parse::<i16>() {
                self.add_token(Token::Integer(val as i32), line, col, 
                    self.stream.position() - start_pos);
            } else if let Ok(val) = num_str.parse::<i32>() {
                self.add_token(Token::Integer(val), line, col, 
                    self.stream.position() - start_pos);
            } else if let Ok(val) = num_str.parse::<i64>() {
                self.add_token(Token::Long(val), line, col, 
                    self.stream.position() - start_pos);
            } else {
                // Too big, use double
                let value: f64 = num_str.parse().map_err(|_| {
                    QError::compile("Invalid numeric literal", line, col)
                })?;
                self.add_token(Token::Double(value), line, col, 
                    self.stream.position() - start_pos);
            }
        }

        Ok(())
    }

    fn scan_hex_octal(&mut self, line: usize, col: usize) -> QResult<()> {
        let start_pos = self.stream.position();
        self.stream.advance(); // Skip &

        match self.stream.peek() {
            Some('H') | Some('h') => {
                // Hexadecimal
                self.stream.advance();
                let num_start = self.stream.position();
                
                while let Some(c) = self.stream.peek() {
                    if c.is_ascii_hexdigit() {
                        self.stream.advance();
                    } else {
                        break;
                    }
                }

                let hex_str: String = self.stream.source[num_start..self.stream.position()]
                    .iter().collect();
                let value = i64::from_str_radix(&hex_str, 16).map_err(|_| {
                    QError::compile("Invalid hexadecimal literal", line, col)
                })?;

                // Check for type suffix
                if let Some(c) = self.stream.peek() {
                    if c == '&' {
                        self.stream.advance();
                        self.add_token(Token::Long(value), line, col, 
                            self.stream.position() - start_pos);
                        return Ok(());
                    }
                }

                // Default to integer if fits, otherwise long
                if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    self.add_token(Token::Integer(value as i32), line, col, 
                        self.stream.position() - start_pos);
                } else {
                    self.add_token(Token::Long(value), line, col, 
                        self.stream.position() - start_pos);
                }
            }
            Some('O') | Some('o') => {
                // Octal
                self.stream.advance();
                let num_start = self.stream.position();
                
                while let Some(c) = self.stream.peek() {
                    if ('0'..='7').contains(&c) {
                        self.stream.advance();
                    } else {
                        break;
                    }
                }

                let oct_str: String = self.stream.source[num_start..self.stream.position()]
                    .iter().collect();
                let value = i64::from_str_radix(&oct_str, 8).map_err(|_| {
                    QError::compile("Invalid octal literal", line, col)
                })?;

                // Check for type suffix
                if let Some(c) = self.stream.peek() {
                    if c == '&' {
                        self.stream.advance();
                        self.add_token(Token::Long(value), line, col, 
                            self.stream.position() - start_pos);
                        return Ok(());
                    }
                }

                // Default to integer if fits, otherwise long
                if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    self.add_token(Token::Integer(value as i32), line, col, 
                        self.stream.position() - start_pos);
                } else {
                    self.add_token(Token::Long(value), line, col, 
                        self.stream.position() - start_pos);
                }
            }
            _ => {
                // Just an & (bitwise AND will be handled differently)
                self.add_token(Token::LongSuffix, line, col, 1);
            }
        }

        Ok(())
    }

    fn scan_identifier(&mut self, line: usize, col: usize) -> QResult<()> {
        let start_pos = self.stream.position();
        
        // Read identifier (letters, digits, underscore, period)
        while let Some(c) = self.stream.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '.' {
                self.stream.advance();
            } else {
                break;
            }
        }

        // Check for type suffix
        let mut suffix: Option<Token> = None;
        if let Some(c) = self.stream.peek() {
            suffix = match c {
                '%' => { self.stream.advance(); Some(Token::IntegerSuffix) }
                '&' => { self.stream.advance(); Some(Token::LongSuffix) }
                '!' => { self.stream.advance(); Some(Token::SingleSuffix) }
                '#' => { self.stream.advance(); Some(Token::DoubleSuffix) }
                '$' => { self.stream.advance(); Some(Token::StringSuffix) }
                _ => None,
            };
        }

        let ident_str: String = self.stream.source[start_pos..self.stream.position()]
            .iter().collect::<String>().to_uppercase();

        // Check for REM comment (special handling)
        if ident_str == "REM" {
            self.stream.skip_line();
            if self.stream.peek() == Some('\n') {
                self.add_token(Token::NewLine, line, col, 1);
                self.stream.advance();
            }
            return Ok(());
        }

        // Check for DATA statement (special handling)
        if ident_str == "DATA" {
            // Read rest of line as data
            let data_start = self.stream.position();
            self.stream.skip_line();
            let data_str: String = self.stream.source[data_start..self.stream.position()]
                .iter().collect();
            self.add_token(Token::Data, line, col, 4);
            self.add_token(Token::String(data_str.trim().to_string()), line, col + 5, 
                self.stream.position() - data_start);
            if self.stream.peek() == Some('\n') {
                self.add_token(Token::NewLine, self.stream.line(), self.stream.column(), 1);
                self.stream.advance();
            }
            return Ok(());
        }

        // Check for LINE INPUT (special keyword)
        if ident_str == "LINE" {
            self.stream.skip_whitespace();
            if let Some(c) = self.stream.peek() {
                if c.is_ascii_alphabetic() {
                    let next_start = self.stream.position();
                    while let Some(c2) = self.stream.peek() {
                        if c2.is_ascii_alphabetic() {
                            self.stream.advance();
                        } else {
                            break;
                        }
                    }
                    let next_str: String = self.stream.source[next_start..self.stream.position()]
                        .iter().collect::<String>().to_uppercase();
                    if next_str == "INPUT" {
                        self.add_token(Token::LineInput, line, col, 
                            self.stream.position() - start_pos);
                        return Ok(());
                    }
                }
            }
        }

        // Check for _UNSIGNED variants (QB64)
        if ident_str == "_UNSIGNED" {
            self.stream.skip_whitespace();
            if let Some(c) = self.stream.peek() {
                if c.is_ascii_alphabetic() || c == '_' {
                    let next_start = self.stream.position();
                    // Check for _INTEGER64
                    if self.stream.peek() == Some('_') {
                        self.stream.advance(); // skip _
                        while let Some(c2) = self.stream.peek() {
                            if c2.is_ascii_alphabetic() || c2.is_ascii_digit() {
                                self.stream.advance();
                            } else {
                                break;
                            }
                        }
                    } else {
                        // Read INTEGER or LONG
                        while let Some(c2) = self.stream.peek() {
                            if c2.is_ascii_alphabetic() {
                                self.stream.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    let next_str: String = self.stream.source[next_start..self.stream.position()]
                        .iter().collect::<String>().to_uppercase();
                    
                    let token = match next_str.as_str() {
                        "INTEGER" => Token::UnsignedIntegerType,
                        "LONG" => Token::UnsignedLongType,
                        "_INTEGER64" => Token::UnsignedInteger64Type,
                        _ => {
                            // Not a recognized unsigned type, treat as identifier
                            self.add_token(Token::Identifier("_UNSIGNED".to_string()), line, col, 
                                self.stream.position() - start_pos);
                            return Ok(());
                        }
                    };
                    self.add_token(token, line, col, self.stream.position() - start_pos);
                    return Ok(());
                }
            }
        }

        // Try to match as keyword
        if let Some(keyword) = string_to_keyword(&ident_str) {
            let token = match (&keyword, suffix) {
                (Token::DefInt, _) | (Token::DefLng, _) | (Token::DefSng, _) |
                (Token::DefDbl, _) | (Token::DefStr, _) => {
                    // DEFxxx statements - keep as is
                    keyword
                }
                (Token::Len, Some(_)) => Token::LenFunc,
                (Token::Date, _) => Token::Date,
                (Token::Time, _) => Token::Time,
                (kw, _) => kw.clone(),
            };
            
            // Check for INPUT#, PRINT#, WRITE# (file I/O)
            let final_token = match token {
                Token::Input | Token::Print | Token::Write => {
                    self.stream.skip_whitespace();
                    if self.stream.peek() == Some('#') {
                        self.stream.advance();
                        match token {
                            Token::Input => Token::InputHash,
                            Token::Print => Token::PrintHash,
                            Token::Write => Token::WriteHash,
                            _ => token,
                        }
                    } else {
                        token
                    }
                }
                _ => token,
            };
            
            self.add_token(final_token, line, col, self.stream.position() - start_pos);
        } else {
            // It's an identifier
            let name = ident_str;
            let token = Token::Identifier(name);
            self.add_token(token, line, col, self.stream.position() - start_pos);
        }

        Ok(())
    }

    fn add_token(&mut self, token: Token, line: usize, col: usize, length: usize) {
        self.tokens.push(TokenInfo::new(token, line, col, length));
    }

    fn scan_metacommand(&mut self, line: usize, col: usize) -> QResult<()> {
        let start_pos = self.stream.position();
        self.stream.advance(); // Skip $
        
        // Read the metacommand name
        while let Some(c) = self.stream.peek() {
            if c.is_ascii_alphabetic() {
                self.stream.advance();
            } else {
                break;
            }
        }
        
        let cmd_str: String = self.stream.source[start_pos..self.stream.position()]
            .iter().collect::<String>().to_uppercase();
        
        // Check for specific metacommands
        let token = match cmd_str.as_str() {
            "$DYNAMIC" => Token::MetaDynamic,
            "$STATIC" => Token::MetaStatic,
            "$INCLUDE" => Token::MetaInclude,
            "$IF" => Token::MetaIf,
            "$ELSE" => Token::MetaElse,
            "$END" => {
                // Check if next is IF
                self.stream.skip_whitespace();
                if self.stream.peek() == Some('I') || self.stream.peek() == Some('i') {
                    let next_start = self.stream.position();
                    while let Some(c) = self.stream.peek() {
                        if c.is_ascii_alphabetic() {
                            self.stream.advance();
                        } else {
                            break;
                        }
                    }
                    let next_str: String = self.stream.source[next_start..self.stream.position()]
                        .iter().collect::<String>().to_uppercase();
                    if next_str == "IF" {
                        Token::MetaEndIf
                    } else {
                        // Just $END, not $END IF
                        Token::MetaEndIf // Treat as endif anyway for simplicity
                    }
                } else {
                    Token::MetaEndIf
                }
            }
            "$RESIZE" => Token::MetaResize,
            "$CONSOLE" => Token::MetaConsole,
            "$SCREENSHOW" => Token::MetaScreenShow,
            "$SCREENHIDE" => Token::ScreenHide,
            _ => {
                // Unknown metacommand, skip it
                return Ok(());
            }
        };
        
        self.add_token(token, line, col, self.stream.position() - start_pos);
        Ok(())
    }
}

/// Convenience function to tokenize source code
pub fn tokenize(source: &str) -> QResult<Vec<TokenInfo>> {
    let scanner = Scanner::new(source);
    scanner.scan_tokens()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let source = "PRINT \"Hello World\"";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 3); // PRINT, "Hello World", EOF
        assert!(matches!(tokens[0].token, Token::Print));
        assert!(matches!(tokens[1].token, Token::String(ref s) if s == "Hello World"));
    }

    #[test]
    fn test_numbers() {
        let source = "10 3.14 &HFF &O77";
        let tokens = tokenize(source).unwrap();
        assert!(matches!(tokens[0].token, Token::Integer(10)));
        // Allow approx_constant for test clarity
        #[allow(clippy::approx_constant)]
        let expected = 3.14;
        assert!(matches!(tokens[1].token, Token::Double(d) if (d - expected).abs() < 0.001));
    }

    #[test]
    fn test_comments() {
        let source = "PRINT 1 ' This is a comment\nPRINT 2";
        let tokens = tokenize(source).unwrap();
        assert!(matches!(tokens[0].token, Token::Print));
        assert!(matches!(tokens[1].token, Token::Integer(1)));
        // After comment parsing, the newline is consumed
        assert!(matches!(tokens[2].token, Token::Print));
        assert!(matches!(tokens[3].token, Token::Integer(2)));
    }
}
