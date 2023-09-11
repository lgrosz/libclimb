//! A grade module for fontainebleau grades.

/// Represents the different modifiers a V grade can have.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Modifier {
    None,
    Plus,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SubValue {
    A,
    B,
    C
}

/// Represents a fontainebleau grade.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Grade {
    pub value: u8,
    pub sub_value: SubValue,
    pub modifier: Modifier,
}

use std::fmt;

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for SubValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubValue::A => write!(f, "a"),
            SubValue::B => write!(f, "b"),
            SubValue::C => write!(f, "c"),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f{}{}{}", self.value, self.sub_value, self.modifier)
    }
}

use std::str::FromStr;

impl FromStr for Modifier {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "+" => Ok(Self::Plus),
            "" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl FromStr for SubValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "a" => Ok(Self::A),
            "A" => Ok(Self::A),
            "b" => Ok(Self::B),
            "B" => Ok(Self::B),
            "c" => Ok(Self::C),
            "C" => Ok(Self::C),
            _ => Err(())
        }
    }
}

use regex::Regex;
impl FromStr for Grade {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        let re = Regex::new(r"(?i)^f?(\d)(a|b|c)(\+?)$").unwrap();

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

        let sub_value = match caps.get(2) {
            Some(m) => m.as_str().parse::<SubValue>()?,
            None => return Err(())
        };

        let modifier = match caps.get(3)  {
            Some(m) => m.as_str().parse::<Modifier>()?,
            None => Modifier::None,
        };

        let grade = Grade {
            value,
            sub_value,
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
        let grade = Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::None };
        assert_eq!(grade.to_string(), "f7a");
    }

    #[test]
    fn to_string_with_modifier() {
        let grade = Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::Plus };
        assert_eq!(grade.to_string(), "f7a+");
    }

    #[test]
    fn from_string() {
        assert_eq!(Grade::from_str("f7a").unwrap(), Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::None } );
    }

    #[test]
    fn from_string_with_modifier() {
        assert_eq!(Grade::from_str("f7a+").unwrap(), Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::Plus } );
    }

    #[test]
    fn from_string_upper() {
        assert_eq!(Grade::from_str("F7a").unwrap(), Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::None } );
    }

    #[test]
    fn from_string_no_prefix() {
        assert_eq!(Grade::from_str("7a+").unwrap(), Grade { value: 7, sub_value: SubValue::A, modifier: Modifier::Plus } );
    }

    #[test]
    fn from_string_invalid() {
        let string = "V10";
        assert!(Grade::from_str(string).is_err());
    }
}
