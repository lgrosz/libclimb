use serde::{Serialize, Deserialize};

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
#[derive(PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Grade {
    level: u8,
    division: Option<Division>,
    plus: bool,
}

/// An enumeration of level subdivisions
///
/// These values subdivide a grading level, giving further refinement to the difficulty represented
/// by a grade. The divisions are only applicable to grade levels greater than 5.
#[derive(PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Division {
    /// Subdivision A, easier than B
    A,

    /// Subdivision B, harder than A, but easier than C
    B,

    /// Subdivision C, harder than B
    C,
}

use std::fmt;

impl Grade {
    /// Creates a new Fontainebleau grade
    ///
    /// `None` will be returned if `division.is_some()` when `level < 6` or if `division.is_none()`
    /// when `level > 5`.
    ///
    /// # Arguments
    ///
    /// * `level` - The numeral level of the grade, as in \[7\]A+.
    /// * `division` - The letter divisor of the grade, as in 7\[A\]+.
    /// * `plus` - Whether or not a "plus" is given.
    pub fn new(level: u8, division: Option<Division>, plus: bool) -> Option<Self> {
        if level < 6 && division.is_some() {
            return None;
        }

        if level > 5 && division.is_none() {
            return None;
        }

        return Some(Grade{ level, division, plus });
    }
}

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Division::A => write!(f, "A"),
            Division::B => write!(f, "B"),
            Division::C => write!(f, "C"),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.level)?;

        match &self.division {
            Some(division) => write!(f, "{}", division),
            _ => Ok(()),
        }?;

        write!(f, "{}", if self.plus { "+" } else { "" })
    }
}

//mod serde;
//pub use serde::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_grade_1() {
        assert!(Grade::new(1, None, false).is_some());
    }

    #[test]
    fn valid_grade_2() {
        assert!(Grade::new(5, None, true).is_some());
    }

    #[test]
    fn valid_grade_3() {
        assert!(Grade::new(6, Some(Division::A), false).is_some());
    }

    #[test]
    fn valid_grade_4() {
        assert!(Grade::new(7, Some(Division::B), true).is_some());
    }

    #[test]
    fn invalid_grade_1() {
        assert!(Grade::new(5, Some(Division::A), false).is_none());
    }

    #[test]
    fn invalid_grade_2() {
        assert!(Grade::new(6, None, false).is_none());
    }

    #[test]
    fn to_string_1() {
        assert_eq!(Grade::new(1, None, false).expect("Good value").to_string(), "1");
    }

    #[test]
    fn to_string_2() {
        assert_eq!(Grade::new(1, None, true).expect("Good value").to_string(), "1+");
    }

    #[test]
    fn to_string_3() {
        assert_eq!(Grade::new(6, Some(Division::A), false).expect("Good value").to_string(), "6A");
    }

    #[test]
    fn to_string_4() {
        assert_eq!(Grade::new(7, Some(Division::B), true).expect("Good value").to_string(), "7B+");
    }

    #[test]
    fn lt_1() {
        assert!(Grade::new(1, None, false).expect("Good value") < Grade::new(2, None, false).expect("Good value"));
    }

    #[test]
    fn lt_2() {
        assert!(Grade::new(1, None, false).expect("Good value") < Grade::new(1, None, true).expect("Good value"));
    }

    #[test]
    fn lt_3() {
        assert!(Grade::new(6, Some(Division::A), false).expect("Good value") < Grade::new(6, Some(Division::B), false).expect("Good value"));
    }

    #[test]
    fn lt_4() {
        assert!(Grade::new(6, Some(Division::A), false).expect("Good value") < Grade::new(6, Some(Division::A), true).expect("Good value"));
    }

    #[test]
    fn json() {
        let grade = Grade::new(6, Some(Division::A), true);

        println!("json = {}", serde_json::to_string(&grade).unwrap());
    }

    #[test]
    fn yaml() {
        let grade = Grade::new(6, Some(Division::A), true);

        println!("yaml = {}", serde_yaml::to_string(&grade).unwrap());
    }
}

