use std::collections::HashMap;
use crate::literal::Literal;
use crate::value::Value;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment{
    pub fn new() -> Self {
        Self{
            values: HashMap::new(),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value){
        self.values.insert(name.to_string(), value);
    }

    pub fn remove(&mut self, name: &str) -> Option<Value>{
        self.values.remove(name)
    }
    
    pub fn get(&self, name: &str) -> Option<&Value>{
        self.values.get(name)
    }
    
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if let Some(existing) = self.values.get_mut(name){
            *existing = value;
            true
        }else {
            false
        }
    }
}