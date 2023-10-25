/// Structure representing a hueco grade.
///
/// A Hueco grade is a difficulty rating on open ended scale. Its levels are represented by
/// positive integers, where the higher the integer the greater than difficulty. There are often
/// modifiers applied to these grades, those being "+" or "-" further dividing each numeric grade.
///
/// When being displayed, the grades are prefixed with a "V," e.g. "V6."
#[derive(PartialEq, Debug)]
pub struct Hueco {
    // The order of these attributes matter for the PartialOrd default trait implementation
    level: u8,
    modifier: Option<Modifier>,
}

/// An enumeration of level modifiers
///
/// These modifiers subdivide a grading level, giving further refinement to the difficulty
/// represented by a grade.
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Modifier {
    /// Modifier to decrease the value of the grade
    Minus,

    /// Modifier to increase the value of the grade
    Plus,
}

use std::{fmt, str::FromStr};

use crate::grades::Grade;

impl Hueco {
    /// Creates a new Hueco grade
    ///
    /// # Arguments
    ///
    /// * `level` - The numeral level of the grade, as in V\[7\].
    /// * `modifier` - The plus or minus modifier of the grade, as in V3\[+\].
    pub fn new(level: u8, modifier: Option<Modifier>) -> Self {
        Hueco { level, modifier }
    }
}

impl Grade for Hueco { }

use std::cmp::Ordering;
impl PartialOrd for Hueco {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare levels when they're not the same
        if self.level != other.level {
            return self.level.partial_cmp(&other.level);
        }

        // There has to be a better way of doing this modifier comparison.. Effectively, this
        // implements the logic "-" < "" < "+"

        // No defined ordering if modifiers are the same
        if self.modifier == other.modifier {
            return None
        }

        // Compare modifiers if they're both defined
        if self.modifier.is_some() && other.modifier.is_some() {
            return self.modifier.partial_cmp(&other.modifier);
        }

        if self.modifier.is_none() {
            if other.modifier == Some(Modifier::Minus) {
                return Some(Ordering::Greater)
            } else if other.modifier == Some(Modifier::Plus) {
                return Some(Ordering::Less)
            }
        }

        if other.modifier.is_none() {
            if self.modifier == Some(Modifier::Minus) {
                return Some(Ordering::Less)
            } else if self.modifier == Some(Modifier::Plus) {
                return Some(Ordering::Greater)
            }
        }

        return None;
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Minus => write!(f, "-"),
            Modifier::Plus => write!(f, "+"),
        }
    }
}

impl fmt::Display for Hueco {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V{}", self.level)?;

        match &self.modifier {
            Some(modifier) => write!(f, "{}", modifier),
            _ => Ok(()),
        }?;

        write!(f, "")
    }
}
#[cfg(test)]
mod display_tests;

impl FromStr for Hueco {
    type Err = HuecoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;

        let re = Regex::new(r"(?i)v?(?P<level>\d+)(?P<modifier>[\+\-])?").unwrap();
        let caps = re.captures(s).ok_or(HuecoParseError::InvalidStr)?;

        let level = caps["level"]
            .parse::<u8>()
            .map_err(|_| HuecoParseError::InvalidStr)?;

        let modifier = match caps.name("modifier") {
            Some(modifier) => {
                let modifier_str = modifier.as_str();
                Some(
                    modifier_str
                    .parse::<Modifier>()
                    .map_err(|_| HuecoParseError::InvalidStr)?,
                    )
            }
            None => None,
        };

        Ok(Hueco::new(level, modifier))
    }
}
#[cfg(test)]
mod from_str_tests;

#[derive(Debug)]
/// Errors for when parsing strings as Hueco grades
pub enum HuecoParseError {
    /// String could not be parsed as a valid Hueco grade string
    InvalidStr,
}

impl FromStr for Modifier {
    type Err = HuecoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
        "-" => Ok(Modifier::Minus),
        "+" => Ok(Modifier::Plus),
        _ => Err(HuecoParseError::InvalidStr)
        }
    }
}


#[cfg(test)]
mod partial_ord_tests;
