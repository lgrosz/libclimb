//! A grade module for boulder, or "B", grades.

/// Represents the possible values of a B grade.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Value {
    One,
    Two,
    Three,
}

/// Represents a B grade.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Grade {
    pub value: Value,
}

use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::One => write!(f, "1"),
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B{}", self.value)
    }
}

use std::str::FromStr;

impl FromStr for Value {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

use regex::Regex;
impl FromStr for Grade {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let re = Regex::new(r"(?i)^b?(\d+)$").unwrap();
        let cap = re.captures(s).unwrap();

        let value_str = cap.get(1).unwrap().as_str();
        let grade = Grade { value: Value::from_str(value_str)? };

        return Ok(grade);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let grade = Grade { value: Value::One };
        assert_eq!(grade.to_string(), "B1");
    }

    #[test]
    fn from_string() {
        assert_eq!(Grade::from_str("B1").unwrap(), Grade { value: Value::One} );
    }

    #[test]
    fn from_string_lower() {
        assert_eq!(Grade::from_str("b1").unwrap(), Grade { value: Value::One} );
    }

    #[test]
    fn from_string_no_prefix() {
        assert_eq!(Grade::from_str("2").unwrap(), Grade { value: Value::Two} );
    }

    #[test]
    fn from_string_invalid() {
        let string = "B4";
        assert!(Grade::from_str(string).is_err());
    }
}
