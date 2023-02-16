#![allow(dead_code)]
use std::fmt;

fn main() -> Result<(), NegativeNotEven> {
    let value = -4;
    let is_even = |x| 0 == x % 2;
    if value < 0 {
        return Err(NegativeNotEven {
            msg: "Negative not even.".to_string(),
        });
    }
    assert_eq!(is_even(value), false);
    Ok(())
}

#[derive(Debug, Default)]
struct NegativeNotEven {
    msg: String,
}

impl NegativeNotEven {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl fmt::Display for NegativeNotEven {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeNotEven {}
