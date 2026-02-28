use crate::ast_nodes::TypeSpec;
use qb_core::data_types::TypeSuffix;

/// Tracks variable declarations and their types
#[derive(Debug, Clone, Default)]
pub struct DeclarationManager {
    // Default type for letters (DEFINT A-Z, etc.)
    default_types: [Option<TypeSuffix>; 26],
    
    // User-defined types
    user_types: std::collections::HashMap<String, Vec<(String, TypeSpec)>>,
    
    // Constants
    constants: std::collections::HashMap<String, crate::ast_nodes::Expression>,
}

impl DeclarationManager {
    pub fn new() -> Self {
        let mut dm = Self::default();
        // Default: all variables are SINGLE
        for i in 0..26 {
            dm.default_types[i] = Some(TypeSuffix::Single);
        }
        dm
    }

    pub fn set_default_type(&mut self, type_char: char, start: char, end: char) {
        let suffix = match type_char {
            'I' | 'i' => TypeSuffix::Integer,
            'L' | 'l' => TypeSuffix::Long,
            'S' | 's' => TypeSuffix::Single,
            'D' | 'd' => TypeSuffix::Double,
            '$' => TypeSuffix::String,
            _ => return,
        };

        let start_idx = (start.to_ascii_uppercase() as u8 - b'A') as usize;
        let end_idx = (end.to_ascii_uppercase() as u8 - b'A') as usize;

        for i in start_idx..=end_idx.min(25) {
            self.default_types[i] = Some(suffix);
        }
    }

    pub fn get_default_type(&self, first_letter: char) -> TypeSuffix {
        let idx = (first_letter.to_ascii_uppercase() as u8 - b'A') as usize;
        if idx < 26 {
            self.default_types[idx].unwrap_or(TypeSuffix::Single)
        } else {
            TypeSuffix::Single
        }
    }

    pub fn infer_type_from_name(&self, name: &str) -> TypeSuffix {
        // Check explicit suffix
        if let Some(last) = name.chars().last() {
            if let Some(suffix) = TypeSuffix::from_char(last) {
                return suffix;
            }
        }
        // Use default type based on first letter
        if let Some(first) = name.chars().next() {
            if first.is_ascii_alphabetic() {
                return self.get_default_type(first);
            }
        }
        TypeSuffix::Single
    }

    pub fn add_user_type(&mut self, name: String, fields: Vec<(String, TypeSpec)>) {
        self.user_types.insert(name.to_uppercase(), fields);
    }

    pub fn get_user_type(&self, name: &str) -> Option<&Vec<(String, TypeSpec)>> {
        self.user_types.get(&name.to_uppercase())
    }

    pub fn add_constant(&mut self, name: String, value: crate::ast_nodes::Expression) {
        self.constants.insert(name.to_uppercase(), value);
    }

    pub fn get_constant(&self, name: &str) -> Option<&crate::ast_nodes::Expression> {
        self.constants.get(&name.to_uppercase())
    }

    pub fn type_spec_to_suffix(&self, spec: &TypeSpec) -> TypeSuffix {
        match spec {
            TypeSpec::Simple(s) => match s.as_str() {
                "INTEGER" => TypeSuffix::Integer,
                "LONG" => TypeSuffix::Long,
                "SINGLE" => TypeSuffix::Single,
                "DOUBLE" => TypeSuffix::Double,
                "STRING" => TypeSuffix::String,
                _ => TypeSuffix::Single,
            }
            TypeSpec::FixedString(_) => TypeSuffix::String,
            TypeSpec::UserDefined(_) => TypeSuffix::Single, // UDTs default
        }
    }
}


