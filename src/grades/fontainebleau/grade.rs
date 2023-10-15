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
        write!(f, "{}", self.level)?;

        match &self.division {
            Some(division) => write!(f, "{}", division),
            _ => Ok(()),
        }?;

        write!(f, "{}", if self.plus { "+" } else { "" })
    }
}

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
mod tests {
    use super::*;

    #[test]
    fn valid_grade_1() {
        assert!(Fontainebleau::new(1, None, false).is_ok());
    }

    #[test]
    fn valid_grade_2() {
        assert!(Fontainebleau::new(5, None, true).is_ok());
    }

    #[test]
    fn valid_grade_3() {
        assert!(Fontainebleau::new(6, Some(Division::A), false).is_ok());
    }

    #[test]
    fn valid_grade_4() {
        assert!(Fontainebleau::new(7, Some(Division::B), true).is_ok());
    }

    #[test]
    fn invalid_grade_1() {
        assert!(matches!(
            Fontainebleau::new(5, Some(Division::A), false).unwrap_err(),
            FontainebleauError::LevelTooLowWithDivision
        ));
    }

    #[test]
    fn invalid_grade_2() {
        assert!(matches!(
            Fontainebleau::new(6, None, false).unwrap_err(),
            FontainebleauError::LevelTooHighWithoutDivision
        ));
    }

    #[test]
    fn to_string_1() {
        assert_eq!(Fontainebleau::new(1, None, false).expect("Good value").to_string(), "1");
    }

    #[test]
    fn to_string_2() {
        assert_eq!(Fontainebleau::new(1, None, true).expect("Good value").to_string(), "1+");
    }

    #[test]
    fn to_string_3() {
        assert_eq!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value").to_string(), "6A");
    }

    #[test]
    fn to_string_4() {
        assert_eq!(Fontainebleau::new(7, Some(Division::B), true).expect("Good value").to_string(), "7B+");
    }

    #[test]
    fn lt_1() {
        assert!(Fontainebleau::new(1, None, false).expect("Good value") < Fontainebleau::new(2, None, false).expect("Good value"));
    }

    #[test]
    fn lt_2() {
        assert!(Fontainebleau::new(1, None, false).expect("Good value") < Fontainebleau::new(1, None, true).expect("Good value"));
    }

    #[test]
    fn lt_3() {
        assert!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value") < Fontainebleau::new(6, Some(Division::B), false).expect("Good value"));
    }

    #[test]
    fn lt_4() {
        assert!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value") < Fontainebleau::new(6, Some(Division::A), true).expect("Good value"));
    }

    #[test]
    fn from_str1() {
        assert!(matches!(
            "".parse::<Fontainebleau>().unwrap_err(),
            FontainebleauParseError::InvalidStr
        ));
    }

    #[test]
    fn from_str2() {
        assert!(matches!(
            "5B+".parse::<Fontainebleau>().unwrap_err(),
            FontainebleauParseError::BadGrade(FontainebleauError::LevelTooLowWithDivision)
        ));
    }

    #[test]
    fn from_str3() {
        assert!(matches!(
            "6".parse::<Fontainebleau>().unwrap_err(),
            FontainebleauParseError::BadGrade(FontainebleauError::LevelTooHighWithoutDivision)
        ));
    }

    #[test]
    fn from_str4() {
        assert_eq!("F8A+".parse::<Fontainebleau>().expect("Good value"), Fontainebleau::new(8, Some(Division::A), true).expect("Good value"))
    }
}
