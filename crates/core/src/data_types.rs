use std::fmt;
use crate::errors::{QError, QErrorCode, QResult};

/// QBasic type suffixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeSuffix {
    Integer,    // %
    Long,       // &
    Single,     // !
    Double,     // #
    String,     // $
    // QB64 extended types
    Integer64,  // && (64-bit signed)
    Float,      // ## (128-bit floating point)
}

impl fmt::Display for TypeSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeSuffix::Integer => write!(f, "%"),
            TypeSuffix::Long => write!(f, "&"),
            TypeSuffix::Single => write!(f, "!"),
            TypeSuffix::Double => write!(f, "#"),
            TypeSuffix::String => write!(f, "$"),
            TypeSuffix::Integer64 => write!(f, "&&"),
            TypeSuffix::Float => write!(f, "##"),
        }
    }
}

impl TypeSuffix {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '%' => Some(TypeSuffix::Integer),
            '&' => Some(TypeSuffix::Long),
            '!' => Some(TypeSuffix::Single),
            '#' => Some(TypeSuffix::Double),
            '$' => Some(TypeSuffix::String),
            _ => None,
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "%" => Some(TypeSuffix::Integer),
            "&" => Some(TypeSuffix::Long),
            "!" => Some(TypeSuffix::Single),
            "#" => Some(TypeSuffix::Double),
            "$" => Some(TypeSuffix::String),
            "&&" => Some(TypeSuffix::Integer64),
            "##" => Some(TypeSuffix::Float),
            _ => None,
        }
    }
}

/// QBasic data types
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum QType {
    // Numeric types (QBasic)
    Integer(i16),
    Long(i32),
    Single(f32),
    Double(f64),
    
    // QB64 extended numeric types
    Integer64(i64),
    
    // Unsigned variants (QB64)
    UnsignedInteger(u16),
    UnsignedLong(u32),
    UnsignedInteger64(u64),
    
    // String types
    String(String),
    FixedString(usize, String),
    
    // User-defined type (raw bytes)
    UserDefined(Vec<u8>),
    
    // Special values
    Empty,
    Null,
}

impl QType {
    /// Get the default value for a type
    pub fn default_value(&self) -> Self {
        match self {
            QType::Integer(_) => QType::Integer(0),
            QType::Long(_) => QType::Long(0),
            QType::Single(_) => QType::Single(0.0),
            QType::Double(_) => QType::Double(0.0),
            // QB64 extended types
            QType::Integer64(_) => QType::Integer64(0),
            QType::UnsignedInteger(_) => QType::UnsignedInteger(0),
            QType::UnsignedLong(_) => QType::UnsignedLong(0),
            QType::UnsignedInteger64(_) => QType::UnsignedInteger64(0),
            QType::String(_) => QType::String(String::new()),
            QType::FixedString(len, _) => QType::FixedString(*len, String::new()),
            QType::UserDefined(bytes) => QType::UserDefined(vec![0; bytes.len()]),
            QType::Empty => QType::Empty,
            QType::Null => QType::Null,
        }
    }

    /// Get the size in bytes
    pub fn size(&self) -> usize {
        match self {
            QType::Integer(_) => 2,
            QType::Long(_) => 4,
            QType::Single(_) => 4,
            QType::Double(_) => 8,
            // QB64 extended types
            QType::Integer64(_) => 8,
            QType::UnsignedInteger(_) => 2,
            QType::UnsignedLong(_) => 4,
            QType::UnsignedInteger64(_) => 8,
            QType::String(s) => 2 + s.len(), // Length prefix + content
            QType::FixedString(len, _) => *len,
            QType::UserDefined(bytes) => bytes.len(),
            QType::Empty => 0,
            QType::Null => 0,
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            QType::Integer(_) => "INTEGER",
            QType::Long(_) => "LONG",
            QType::Single(_) => "SINGLE",
            QType::Double(_) => "DOUBLE",
            QType::Integer64(_) => "_INTEGER64",
            QType::UnsignedInteger(_) => "_UNSIGNED INTEGER",
            QType::UnsignedLong(_) => "_UNSIGNED LONG",
            QType::UnsignedInteger64(_) => "_UNSIGNED _INTEGER64",
            QType::String(_) => "STRING",
            QType::FixedString(_, _) => "STRING*n",
            QType::UserDefined(_) => "USER DEFINED",
            QType::Empty => "EMPTY",
            QType::Null => "NULL",
        }
    }

    /// Check if the value is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, QType::Integer(_) | QType::Long(_) | QType::Single(_) | QType::Double(_) |
                 QType::Integer64(_) | QType::UnsignedInteger(_) | QType::UnsignedLong(_) | 
                 QType::UnsignedInteger64(_))
    }

    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, QType::String(_) | QType::FixedString(_, _))
    }

    /// Convert to integer
    pub fn to_integer(&self) -> QResult<i16> {
        match self {
            QType::Integer(v) => Ok(*v),
            QType::Long(v) => Ok(*v as i16),
            QType::Single(v) => Ok(*v as i16),
            QType::Double(v) => Ok(*v as i16),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Convert to long
    pub fn to_long(&self) -> QResult<i32> {
        match self {
            QType::Integer(v) => Ok(*v as i32),
            QType::Long(v) => Ok(*v),
            QType::Single(v) => Ok(*v as i32),
            QType::Double(v) => Ok(*v as i32),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Convert to single
    pub fn to_single(&self) -> QResult<f32> {
        match self {
            QType::Integer(v) => Ok(*v as f32),
            QType::Long(v) => Ok(*v as f32),
            QType::Single(v) => Ok(*v),
            QType::Double(v) => Ok(*v as f32),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Convert to double
    pub fn to_double(&self) -> QResult<f64> {
        match self {
            QType::Integer(v) => Ok(*v as f64),
            QType::Long(v) => Ok(*v as f64),
            QType::Single(v) => Ok(*v as f64),
            QType::Double(v) => Ok(*v),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Convert to string
    pub fn to_qstring(&self) -> QResult<String> {
        match self {
            QType::String(s) => Ok(s.clone()),
            QType::FixedString(_, s) => Ok(s.clone()),
            QType::Integer(v) => Ok(v.to_string()),
            QType::Long(v) => Ok(v.to_string()),
            QType::Single(v) => Ok(v.to_string()),
            QType::Double(v) => Ok(v.to_string()),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Negate the value
    pub fn negate(&self) -> QResult<QType> {
        match self {
            QType::Integer(v) => Ok(QType::Integer(-v)),
            QType::Long(v) => Ok(QType::Long(-v)),
            QType::Single(v) => Ok(QType::Single(-v)),
            QType::Double(v) => Ok(QType::Double(-v)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Add two values
    pub fn add(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            // String concatenation
            (QType::String(a), QType::String(b)) => Ok(QType::String(format!("{}{}", a, b))),
            (QType::String(a), b) => Ok(QType::String(format!("{}{}", a, b.to_qstring()?))),
            (a, QType::String(b)) => Ok(QType::String(format!("{}{}", a.to_qstring()?, b))),
            
            // Numeric addition with promotion
            (QType::Double(a), b) => Ok(QType::Double(a + b.to_double()?)),
            (a, QType::Double(b)) => Ok(QType::Double(a.to_double()? + b)),
            (QType::Single(a), b) => Ok(QType::Single(a + b.to_single()?)),
            (a, QType::Single(b)) => Ok(QType::Single(a.to_single()? + b)),
            (QType::Long(a), b) => Ok(QType::Long(a + b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? + b)),
            (QType::Integer(a), QType::Integer(b)) => {
                let result = a.wrapping_add(*b);
                Ok(QType::Integer(result))
            }
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Subtract two values
    pub fn subtract(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            (QType::Double(a), b) => Ok(QType::Double(a - b.to_double()?)),
            (a, QType::Double(b)) => Ok(QType::Double(a.to_double()? - b)),
            (QType::Single(a), b) => Ok(QType::Single(a - b.to_single()?)),
            (a, QType::Single(b)) => Ok(QType::Single(a.to_single()? - b)),
            (QType::Long(a), b) => Ok(QType::Long(a - b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? - b)),
            (QType::Integer(a), QType::Integer(b)) => {
                let result = a.wrapping_sub(*b);
                Ok(QType::Integer(result))
            }
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Multiply two values
    pub fn multiply(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            (QType::Double(a), b) => Ok(QType::Double(a * b.to_double()?)),
            (a, QType::Double(b)) => Ok(QType::Double(a.to_double()? * b)),
            (QType::Single(a), b) => Ok(QType::Single(a * b.to_single()?)),
            (a, QType::Single(b)) => Ok(QType::Single(a.to_single()? * b)),
            (QType::Long(a), b) => Ok(QType::Long(a * b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? * b)),
            (QType::Integer(a), QType::Integer(b)) => {
                let result = a.wrapping_mul(*b);
                Ok(QType::Integer(result))
            }
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Divide two values
    pub fn divide(&self, other: &QType) -> QResult<QType> {
        let divisor = other.to_double()?;
        if divisor == 0.0 {
            return Err(QError::runtime(QErrorCode::DivisionByZero, 0, 0));
        }
        Ok(QType::Double(self.to_double()? / divisor))
    }

    /// Integer divide
    pub fn int_divide(&self, other: &QType) -> QResult<QType> {
        let divisor = other.to_long()?;
        if divisor == 0 {
            return Err(QError::runtime(QErrorCode::DivisionByZero, 0, 0));
        }
        Ok(QType::Long(self.to_long()? / divisor))
    }

    /// Modulo
    pub fn modulo(&self, other: &QType) -> QResult<QType> {
        let divisor = other.to_long()?;
        if divisor == 0 {
            return Err(QError::runtime(QErrorCode::DivisionByZero, 0, 0));
        }
        Ok(QType::Long(self.to_long()? % divisor))
    }

    /// Power
    pub fn power(&self, other: &QType) -> QResult<QType> {
        let base = self.to_double()?;
        let exp = other.to_double()?;
        Ok(QType::Double(base.powf(exp)))
    }

    /// Compare two values
    pub fn compare(&self, other: &QType, op: CompareOp) -> QResult<bool> {
        let result = match (self, other) {
            (QType::String(a), QType::String(b)) => match op {
                CompareOp::Eq => a == b,
                CompareOp::Ne => a != b,
                CompareOp::Lt => a < b,
                CompareOp::Le => a <= b,
                CompareOp::Gt => a > b,
                CompareOp::Ge => a >= b,
            }
            (a, b) if a.is_numeric() && b.is_numeric() => {
                let a = a.to_double()?;
                let b = b.to_double()?;
                match op {
                    CompareOp::Eq => (a - b).abs() < f64::EPSILON,
                    CompareOp::Ne => (a - b).abs() >= f64::EPSILON,
                    CompareOp::Lt => a < b,
                    CompareOp::Le => a <= b,
                    CompareOp::Gt => a > b,
                    CompareOp::Ge => a >= b,
                }
            }
            _ => return Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        };
        Ok(result)
    }

    /// Bitwise NOT
    pub fn bitwise_not(&self) -> QResult<QType> {
        match self {
            QType::Integer(v) => Ok(QType::Integer(!v)),
            QType::Long(v) => Ok(QType::Long(!v)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Bitwise AND
    pub fn bitwise_and(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            (QType::Long(a), b) => Ok(QType::Long(a & b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? & b)),
            (QType::Integer(a), QType::Integer(b)) => Ok(QType::Integer(a & b)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Bitwise OR
    pub fn bitwise_or(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            (QType::Long(a), b) => Ok(QType::Long(a | b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? | b)),
            (QType::Integer(a), QType::Integer(b)) => Ok(QType::Integer(a | b)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Bitwise XOR
    pub fn bitwise_xor(&self, other: &QType) -> QResult<QType> {
        match (self, other) {
            (QType::Long(a), b) => Ok(QType::Long(a ^ b.to_long()?)),
            (a, QType::Long(b)) => Ok(QType::Long(a.to_long()? ^ b)),
            (QType::Integer(a), QType::Integer(b)) => Ok(QType::Integer(a ^ b)),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    /// Bitwise IMP (implication)
    pub fn bitwise_imp(&self, other: &QType) -> QResult<QType> {
        // A IMP B = NOT A OR B
        self.bitwise_not()?.bitwise_or(other)
    }

    /// Bitwise EQV (equivalence)
    pub fn bitwise_eqv(&self, other: &QType) -> QResult<QType> {
        // A EQV B = NOT (A XOR B)
        self.bitwise_xor(other)?.bitwise_not()
    }

    // Mathematical functions
    pub fn math_abs(&self) -> QResult<QType> {
        match self {
            QType::Double(v) => Ok(QType::Double(v.abs())),
            QType::Single(v) => Ok(QType::Single(v.abs())),
            QType::Long(v) => Ok(QType::Long(v.abs())),
            QType::Integer(v) => Ok(QType::Integer(v.abs())),
            _ => Err(QError::runtime(QErrorCode::TypeMismatch, 0, 0)),
        }
    }

    pub fn math_sgn(&self) -> QResult<QType> {
        let n = self.to_double()?;
        let val = if n > 0.0 { 1 } else if n < 0.0 { -1 } else { 0 };
        Ok(QType::Integer(val))
    }

    pub fn math_int(&self) -> QResult<QType> {
        Ok(QType::Double(self.to_double()?.floor()))
    }

    pub fn math_fix(&self) -> QResult<QType> {
        Ok(QType::Double(self.to_double()?.trunc()))
    }

    pub fn math_sqr(&self) -> QResult<QType> {
        let n = self.to_double()?;
        if n < 0.0 {
            Err(QError::runtime(QErrorCode::IllegalFunctionCall, 0, 0))
        } else {
            Ok(QType::Double(n.sqrt()))
        }
    }

    pub fn math_sin(&self) -> QResult<QType> { Ok(QType::Double(self.to_double()?.sin())) }
    pub fn math_cos(&self) -> QResult<QType> { Ok(QType::Double(self.to_double()?.cos())) }
    pub fn math_tan(&self) -> QResult<QType> { Ok(QType::Double(self.to_double()?.tan())) }
    pub fn math_atn(&self) -> QResult<QType> { Ok(QType::Double(self.to_double()?.atan())) }
    pub fn math_exp(&self) -> QResult<QType> { Ok(QType::Double(self.to_double()?.exp())) }
    
    pub fn math_log(&self) -> QResult<QType> {
        let n = self.to_double()?;
        if n <= 0.0 {
            Err(QError::runtime(QErrorCode::IllegalFunctionCall, 0, 0))
        } else {
            Ok(QType::Double(n.ln()))
        }
    }
}

impl fmt::Display for QType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QType::Integer(v) => write!(f, "{}", v),
            QType::Long(v) => write!(f, "{}", v),
            QType::Single(v) => write!(f, "{}", v),
            QType::Double(v) => write!(f, "{}", v),
            // QB64 extended types
            QType::Integer64(v) => write!(f, "{}", v),
            QType::UnsignedInteger(v) => write!(f, "{}", v),
            QType::UnsignedLong(v) => write!(f, "{}", v),
            QType::UnsignedInteger64(v) => write!(f, "{}", v),
            QType::String(s) => write!(f, "{}", s),
            QType::FixedString(_, s) => write!(f, "{}", s),
            QType::UserDefined(_) => write!(f, "<UDT>"),
            QType::Empty => write!(f, ""),
            QType::Null => write!(f, "<NULL>"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq, // =
    Ne, // <>
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=
}

/// Variable identifier with optional type suffix
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId {
    pub name: String,
    pub suffix: Option<TypeSuffix>,
}

impl VariableId {
    pub fn new(name: impl Into<String>, suffix: Option<TypeSuffix>) -> Self {
        Self { name: name.into(), suffix }
    }

    pub fn full_name(&self) -> String {
        match &self.suffix {
            Some(s) => format!("{}{}", self.name, s).to_uppercase(),
            None => self.name.to_uppercase(),
        }
    }
}

/// Array bounds for DIM statement
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayBounds {
    pub lower: i32,
    pub upper: i32,
}

impl ArrayBounds {
    pub fn new(lower: i32, upper: i32) -> Self {
        Self { lower, upper }
    }

    pub fn single(upper: i32) -> Self {
        Self { lower: 0, upper }
    }

    pub fn count(&self) -> usize {
        ((self.upper - self.lower) + 1) as usize
    }

    pub fn is_in_bounds(&self, index: i32) -> bool {
        index >= self.lower && index <= self.upper
    }
}

/// Variable reference (scalar or array element)
#[derive(Debug, Clone, PartialEq)]
pub enum VariableRef {
    Scalar(VariableId),
    Array(VariableId, Vec<QType>), // Variable and index values
}

/// Function/Sub parameter type
#[derive(Debug, Clone, PartialEq)]
pub enum ParamType {
    ByVal(VariableId),  // Pass by value
    ByRef(VariableId),  // Pass by reference
}

/// User-defined type definition
#[derive(Debug, Clone, PartialEq)]
pub struct UserTypeDef {
    pub name: String,
    pub fields: Vec<(String, QType)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_math_abs() {
        let neg_int = QType::Integer(-5);
        assert_eq!(neg_int.math_abs().unwrap(), QType::Integer(5));
        
        let pos_dbl = QType::Double(3.14);
        assert_eq!(pos_dbl.math_abs().unwrap(), QType::Double(3.14));
    }

    #[test]
    fn test_math_sgn() {
        let pos = QType::Single(5.5);
        let zero = QType::Integer(0);
        let neg = QType::Double(-2.2);

        assert_eq!(pos.math_sgn().unwrap(), QType::Integer(1));
        assert_eq!(zero.math_sgn().unwrap(), QType::Integer(0));
        assert_eq!(neg.math_sgn().unwrap(), QType::Integer(-1));
    }

    #[test]
    fn test_math_sqr() {
        let val = QType::Double(16.0);
        assert_eq!(val.math_sqr().unwrap(), QType::Double(4.0));

        let neg_val = QType::Double(-1.0);
        assert!(neg_val.math_sqr().is_err()); // Illegal function call
    }

    #[test]
    fn test_math_int_fix() {
        let val1 = QType::Single(2.8);
        let val2 = QType::Single(-2.8);

        assert_eq!(val1.math_int().unwrap(), QType::Double(2.0));
        assert_eq!(val2.math_int().unwrap(), QType::Double(-3.0));

        assert_eq!(val1.math_fix().unwrap(), QType::Double(2.0));
        assert_eq!(val2.math_fix().unwrap(), QType::Double(-2.0));
    }

    #[test]
    fn test_math_log() {
        let e = QType::Double(std::f64::consts::E);
        assert!((e.math_log().unwrap().to_double().unwrap() - 1.0).abs() < f64::EPSILON);
        
        let zero = QType::Double(0.0);
        assert!(zero.math_log().is_err());
    }
}
