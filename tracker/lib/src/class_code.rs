use regex::Regex;
use std::{fmt, rc::Rc};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[A-Z]{4}\d{3}$").unwrap();
}

/// String wrapper to enforce the Class Code invariant.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClassCodes(pub Vec<Rc<ClassCode>>);

impl ClassCodes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&mut self, s: &str) -> Result<Rc<ClassCode>, &'static str> {
        if let Some(c) = self.0.iter().find(|r| r.0 == s) {
            return Ok(Rc::clone(c));
        }

        let cc = ClassCode::new(s)?;
        let rc = Rc::new(cc);
        self.0.push(rc);
        Ok(Rc::clone(self.0.last().unwrap()))
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

    #[test]
    fn class_codes_1() {
        let mut codes = ClassCodes::new();

        // get multiple times but the actual number of ClassCode instances is 1
        let a = codes.get("TEST111");
        let b = codes.get("TEST111");
        assert!(a.is_ok());
        assert!(b.is_ok());
        assert_eq!(1, codes.0.len());
    }

    #[test]
    fn class_codes_2() {
        let mut codes = ClassCodes::new();

        // 5 different class codes creates 5 instances
        let _ = codes.get("TEST001");
        let _ = codes.get("TEST002");
        let _ = codes.get("TEST003");
        let _ = codes.get("TEST004");
        let _ = codes.get("TEST005");

        assert_eq!(5, codes.0.len());
    }
}
