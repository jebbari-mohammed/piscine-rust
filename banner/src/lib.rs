use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let short_hand = format!("-{}", name.chars().next().unwrap());
        let long_hand = format!("--{}", name);
        Flag {
            short_hand,
            long_hand,
            desc: d.to_string(),
        }
    }
}


pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        todo!()
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        todo!()
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}