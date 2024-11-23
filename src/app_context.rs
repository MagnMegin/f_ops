use std::collections::HashMap;
use core::f32::consts;

pub struct Context {
    vars: HashMap<String,f32>,
}

impl Context {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("pi".to_string(), consts::PI);
        vars.insert("e".to_string(), consts::E);

        Context{vars}
    }

    pub fn var(&self, var_name: &str) -> Option<f32> {
        self.vars.get(var_name).copied()
    }
    
    pub fn set_var(&mut self, var_name: &str, value: f32) {

        self.vars.insert(var_name.to_string(), value);
    }

    pub fn call_func(&self, func_name: &str, arg: f32) -> Option<f32> {
        match func_name {
            "sqrt" => Some(arg.sqrt()),
            "sin" => Some(arg.sin()),
            "cos" => Some(arg.cos()),
            "tan" => Some(arg.tan()),
            "exp" => Some(arg.exp()),
            "ln" => Some(arg.ln()),
            _ => None,
        }
    }
}