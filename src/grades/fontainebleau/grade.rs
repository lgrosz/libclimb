/// Structure representing a fontainebleau grade.
///
/// A Fontainebleau grade is a difficulty rating on open ended scale. Its levels are represented by
/// positive integers, where the higher the integer the greater than difficulty. After level 6, the
/// grades are divided into three catagories, A, B, and C, each denoting a slightly higher degree
/// of difficulty. At any level, the grade may be postfixed with a "+", refining the difficulty
/// between the two adjacent steps.
///
/// These grades can be displayed to the user with the numeral level, the captital-alphabetic
/// division (if applicable), followed the "+.". For example, 4, 4+, 6a, 7a+, are all valid grades
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Fontainebleau {
    // The order of these attributes matter for the PartialOrd default trait implementation
    level: u8,
    division: Option<Division>,
    plus: bool,
}

/// An enumeration of level subdivisions
///
/// These values subdivide a grading level, giving further refinement to the difficulty represented
/// by a grade. The divisions are only applicable to grade levels greater than 5.
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Division {
    /// Subdivision A, easier than B
    A,

    /// Subdivision B, harder than A, but easier than C
    B,

    /// Subdivision C, harder than B
    C,
}

use std::{fmt, str::FromStr};

use crate::grades::Grade;

#[derive(Debug)]
/// Errors related to a Fontainebleau grade
pub enum FontainebleauError {
    /// Thrown when the level is lower than 6 and contains a division
    LevelTooLowWithDivision,

    /// Thrown when the level is higher than 5 and does not contain a division
    LevelTooHighWithoutDivision,
}

impl Fontainebleau {
    /// Creates a new Fontainebleau grade
    ///
    /// Every level greater than 5 *must* have a division. And every level less than 6 *mustn't*
    /// have a division.
    ///
    /// # Arguments
    ///
    /// * `level` - The numeral level of the grade, as in \[7\]A+.
    /// * `division` - The letter divisor of the grade, as in 7\[A\]+.
    /// * `plus` - Whether or not a "plus" is given.
    pub fn new(level: u8, division: Option<Division>, plus: bool) -> Result<Self, FontainebleauError> {
        if level < 6 && division.is_some() {
            return Err(FontainebleauError::LevelTooLowWithDivision);
        }

        if level > 5 && division.is_none() {
            return Err(FontainebleauError::LevelTooHighWithoutDivision);
        }

        Ok(Fontainebleau { level, division, plus })
    }
}
#[cfg(test)]
mod impl_tests;


impl Grade for Fontainebleau { }

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Division::A => write!(f, "A"),
            Division::B => write!(f, "B"),
            Division::C => write!(f, "C"),
        }
    }
}

impl fmt::Display for Fontainebleau {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F{}", self.level)?;

        match &self.division {
            Some(division) => write!(f, "{}", division),
            _ => Ok(()),
        }?;

        write!(f, "{}", if self.plus { "+" } else { "" })
    }
}
#[cfg(test)]
mod display_tests;

impl FromStr for Fontainebleau {
    type Err = FontainebleauParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;

        let re = Regex::new(r"(?i)f?(?P<level>\d+)(?P<division>[abc])?(?P<plus>\+)?").unwrap();
        let caps = re.captures(s).ok_or(FontainebleauParseError::InvalidStr)?;

        let level = caps["level"]
            .parse::<u8>()
            .map_err(|_| FontainebleauParseError::InvalidStr)?;

        let division = match caps.name("division") {
            Some(division) => {
                let division_str = division.as_str();
                Some(
                    division_str
                    .parse::<Division>()
                    .map_err(|_| FontainebleauParseError::InvalidStr)?,
                    )
            }
            None => None,
        };

        let plus = caps.name("plus").is_some();

        Fontainebleau::new(level, division, plus)
            .map_err(|e| FontainebleauParseError::BadGrade(e))
    }
}
#[cfg(test)]
mod from_str_tests;

#[derive(Debug)]
/// Errors for when parsing strings as Fontainebleau grades
pub enum FontainebleauParseError {
    /// Propogation of FontainebleauError
    BadGrade(FontainebleauError),

    /// String could not be parsed as a valid Fontainebleau grade string
    InvalidStr,
}

impl FromStr for Division {
    type Err = FontainebleauParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
        "a" => Ok(Division::A),
        "b" => Ok(Division::B),
        "c" => Ok(Division::C),
        _ => Err(FontainebleauParseError::InvalidStr)
        }
    }
}


#[cfg(test)]
mod partial_ord_tests;
