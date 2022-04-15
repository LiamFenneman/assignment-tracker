use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[A-Z]{4}\d{3}$").unwrap();
}

/// String wrapper to enforce the Class Code invariant.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClassCode(String);

impl ClassCode {
    pub fn new(str: &str) -> Result<Self, &'static str> {
        if !RE.is_match(str) {
            return Err("Given string does not follow the correct format");
        }

        Ok(Self(str.to_string()))
    }

    pub fn get(&self) -> &str {
        return &self.0;
    }
}

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_1() {
        let cc = ClassCode::new("TEST101");
        assert!(cc.is_ok());
        assert_eq!("TEST101", cc.unwrap().0);
    }

    #[test]
    fn test_valid_2() {
        let cc = ClassCode::new("SOME999");
        assert!(cc.is_ok());
        assert_eq!("SOME999", cc.unwrap().0);
    }

    #[test]
    fn test_invalid() {
        let cc = ClassCode::new("");
        assert!(cc.is_err());
    }

    #[test]
    fn test_invalid_2() {
        let cc = ClassCode::new("not a class code");
        assert!(cc.is_err());
        let cc = ClassCode::new("TEST");
        assert!(cc.is_err());
    }

    #[test]
    fn test_invalid_3() {
        let cc = ClassCode::new("code123");
        assert!(cc.is_err());
    }

    #[test]
    fn test_invalid_4() {
        let cc = ClassCode::new("CLASS101");
        assert!(cc.is_err());
        let cc = ClassCode::new("CLASS11");
        assert!(cc.is_err());
    }
}
