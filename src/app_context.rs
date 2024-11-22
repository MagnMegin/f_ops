use std::collections::HashMap;

pub struct Context {
    vars: HashMap<String,f32>,
    // To-Do: Add functions
}

impl Context {
    pub fn new() -> Self {
        Context{vars: HashMap::new()}
    }

    pub fn var(&self, var_name: &str) -> Option<f32> {
        self.vars.get(var_name).copied()
    }
    
    pub fn set_var(&mut self, var_name: &str, value: f32) {
        self.vars.insert(var_name.to_string(), value);
    }   
}