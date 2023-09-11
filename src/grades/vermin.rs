//! A grade module for vermin, or "V", grades.

/// Represents the different modifiers a V grade can have.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Modifier {
    None,
    Plus,
    Minus,
}

/// Represents a V grade.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Grade {
    pub value: u8,
    pub modifier: Modifier,
}

use std::fmt;

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::Minus => write!(f, "-"),
            Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V{}{}", self.value, self.modifier)
    }
}

use std::str::FromStr;

impl FromStr for Modifier {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "-" => Ok(Self::Minus),
            "+" => Ok(Self::Plus),
            "" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

use regex::Regex;
impl FromStr for Grade {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        let re = Regex::new(r"(?i)^v?(\d+)(\+|-)?$").unwrap();

        let some_caps = re.captures(s);
        if some_caps.is_none() {
            return Err(());
        }

        let caps = some_caps.unwrap();

        // Since the regex was matched, these unwraps will succeed
        let value = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        let modifier = match caps.get(2)  {
            Some(m) => m.as_str().parse::<Modifier>()?,
            None => Modifier::None,
        };

        let grade = Grade {
            value,
            modifier,
        };

        return Ok(grade);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let grade = Grade { value: 5, modifier: Modifier::None };
        assert_eq!(grade.to_string(), "V5");
    }

    #[test]
    fn to_string_with_modifier() {
        let grade = Grade { value: 5, modifier: Modifier::Plus };
        assert_eq!(grade.to_string(), "V5+");
    }

    #[test]
    fn from_string() {
        assert_eq!(Grade::from_str("V5").unwrap(), Grade { value: 5, modifier: Modifier::None } );
    }

    #[test]
    fn from_string_with_modifier() {
        assert_eq!(Grade::from_str("V5+").unwrap(), Grade { value: 5, modifier: Modifier::Plus } );
    }

    #[test]
    fn from_string_lower() {
        assert_eq!(Grade::from_str("v1").unwrap(), Grade { value: 1, modifier: Modifier::None } );
    }

    #[test]
    fn from_string_no_prefix() {
        assert_eq!(Grade::from_str("2").unwrap(), Grade { value: 2, modifier: Modifier::None } );
    }

    #[test]
    fn from_string_invalid() {
        let string = "B1";
        assert!(Grade::from_str(string).is_err());
    }
}
