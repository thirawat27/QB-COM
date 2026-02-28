use qb_core::data_types::QType;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Scope for variable tracking
#[derive(Debug, Clone)]
pub struct Scope {
    variables: IndexMap<String, QType>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: IndexMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Box<Scope>) -> Self {
        Self {
            variables: IndexMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, type_: QType) {
        self.variables.insert(name.into(), type_);
    }

    pub fn lookup(&self, name: &str) -> Option<&QType> {
        if let Some(type_) = self.variables.get(name) {
            Some(type_)
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut QType> {
        if self.variables.contains_key(name) {
            self.variables.get_mut(name)
        } else if let Some(ref mut parent) = self.parent {
            parent.lookup_mut(name)
        } else {
            None
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol table for the entire program
#[derive(Debug)]
pub struct SymbolTable {
    global_scope: Scope,
    scopes: Vec<Scope>,
    functions: HashMap<String, (Vec<QType>, QType)>, // name -> (param_types, return_type)
    subroutines: HashMap<String, Vec<QType>>,       // name -> param_types
    line_numbers: HashMap<u32, usize>,              // line number -> statement index
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            global_scope: Scope::new(),
            scopes: Vec::new(),
            functions: HashMap::new(),
            subroutines: HashMap::new(),
            line_numbers: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Scope::with_parent(Box::new(self.current_scope().clone()));
        self.scopes.push(new_scope);
    }

    pub fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            if let Some(parent) = scope.parent {
                if self.scopes.is_empty() {
                    self.global_scope = *parent;
                }
            }
        }
    }

    pub fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap_or(&self.global_scope)
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap_or(&mut self.global_scope)
    }

    pub fn define_variable(&mut self, name: impl Into<String>, type_: QType) {
        self.current_scope_mut().define(name, type_);
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&QType> {
        self.current_scope().lookup(name)
    }

    pub fn define_function(&mut self, name: impl Into<String>, params: Vec<QType>, return_type: QType) {
        self.functions.insert(name.into(), (params, return_type));
    }

    pub fn lookup_function(&self, name: &str) -> Option<&(Vec<QType>, QType)> {
        self.functions.get(&name.to_uppercase())
    }

    pub fn define_subroutine(&mut self, name: impl Into<String>, params: Vec<QType>) {
        self.subroutines.insert(name.into(), params);
    }

    pub fn lookup_subroutine(&self, name: &str) -> Option<&Vec<QType>> {
        self.subroutines.get(&name.to_uppercase())
    }

    pub fn add_line_number(&mut self, number: u32, index: usize) {
        self.line_numbers.insert(number, index);
    }

    pub fn get_line_index(&self, number: u32) -> Option<usize> {
        self.line_numbers.get(&number).copied()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
