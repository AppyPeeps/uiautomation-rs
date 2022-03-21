use std::fmt::Display;

use windows::Win32::System::Com::VARIANT;

use crate::Error;
use crate::Result;

#[derive(Debug)]
pub enum Value {
    INT(i32),
    LONG(i64),
    STR(String)
}

impl Value {
    pub fn as_int(&self) -> i32 {
        match self {
            Value::INT(value) => *value,
            Value::LONG(value) => *value as i32,
            Value::STR(value) => value.parse::<i32>().unwrap_or(0),
        }
    }

    pub fn as_long(&self) -> i64 {
        match self {
            Value::INT(value) => *value as i64,
            Value::LONG(value) => *value,
            Value::STR(value) => value.parse::<i64>().unwrap_or(0),
        }
    }

    pub fn as_string(&self) -> String{
        match self {
            Value::INT(value) => format!("{}", value),
            Value::LONG(value) => format!("{}", value),
            Value::STR(value) => value.clone(),
        }
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::INT(value as i32)
    }
}
impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::INT(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::LONG(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::STR(String::from(value))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(value) => write!(f, "{}", value),
            Value::LONG(value) => write!(f, "{}", value),
            Value::STR(value) => write!(f, "{}", value),
        }
    }
}

/// A Wrapper for windows `VARIANT`
#[derive(Clone, PartialEq, Eq)]
pub struct Variant {
    value: VARIANT
}

impl Default for Variant {
    fn default() -> Self {
        Self { 
            value: Default::default() 
        }
    }
}
