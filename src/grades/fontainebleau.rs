#[derive(Debug)]
#[derive(PartialEq)]
pub enum Modifier {
    None,
    Plus,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Grade {
    pub numeral: u8,
    pub letter: char,
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

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f{}{}{}", self.numeral, self.letter, self.modifier)
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
        let numeral = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        let letter = caps
            .get(2)
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap();

        let modifier = match caps.get(3)  {
            Some(m) => m.as_str().parse::<Modifier>()?,
            None => Modifier::None,
        };

        let grade = Grade {
            numeral,
            letter,
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
        let grade = Grade { numeral: 7, letter: 'a', modifier: Modifier::None };
        assert_eq!(grade.to_string(), "f7a");
    }

    #[test]
    fn to_string_with_modifier() {
        let grade = Grade { numeral: 7, letter: 'a', modifier: Modifier::Plus };
        assert_eq!(grade.to_string(), "f7a+");
    }

    #[test]
    fn from_string() {
        assert_eq!(Grade::from_str("f7a").unwrap(), Grade { numeral: 7, letter: 'a', modifier: Modifier::None } );
    }

    #[test]
    fn from_string_with_modifier() {
        assert_eq!(Grade::from_str("f7a+").unwrap(), Grade { numeral: 7, letter:'a', modifier: Modifier::Plus } );
    }

    #[test]
    fn from_string_upper() {
        assert_eq!(Grade::from_str("F7a").unwrap(), Grade { numeral: 7, letter: 'a', modifier: Modifier::None } );
    }

    #[test]
    fn from_string_no_prefix() {
        assert_eq!(Grade::from_str("7a+").unwrap(), Grade { numeral: 7, letter: 'a', modifier: Modifier::Plus } );
    }

    #[test]
    fn from_string_invalid() {
        let string = "V10";
        assert!(Grade::from_str(string).is_err());
    }
}
