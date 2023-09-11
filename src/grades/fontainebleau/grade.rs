/// Structure representing a fontainebleau grade.
#[derive(PartialEq, PartialOrd)]
pub struct Grade {
    // numerals can range from 1-9
    numeral: u8,

    // for numerals >5, letters a, b, and c can exist
    letter: Option<char>,

    // further refinement between grades
    plus: bool,
}

use std::fmt;

impl Grade {
    /// Creates a new Fontainebleau grade
    ///
    /// Creates a Fontainbleau grade which, in addition to what the types enforce, satisfies the
    /// following requirements
    /// - `numeral` < 10
    /// - `letter` ∈ { A, B, C, ∅ }, case insensitive
    /// - `letter` may only be present if `numeral` < 6
    /// - `letter` must be present if `numeral` > 5
    ///
    /// # Arguments
    ///
    /// * `numeral` - The numeral grade, \[7\]B+, `numeral < 10`.
    /// * `letter` - The letter grade, 7\[B\]+, if applicable
    /// * `plus` - Whether or not the plus exists, `true` with 7B\[+\] or false with 7B\[\]
    pub fn new(numeral: u8, letter: Option<char>, plus: bool) -> Option<Self> {
        if numeral > 9 {
            return None;
        }

        if numeral < 6 && letter.is_some() {
            return None;
        }

        if numeral > 5 && letter.is_none() {
            return None;
        }

        if letter.is_some() &&
           letter.unwrap().to_ascii_lowercase() != 'a' &&
           letter.unwrap().to_ascii_lowercase() != 'b' &&
           letter.unwrap().to_ascii_lowercase() != 'c' {
            return None;
        }

        return Some(Grade{ numeral, letter, plus });
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let plus = if self.plus { "+" } else { "" };
        let letter = if self.letter.is_some() { self.letter.unwrap().to_ascii_uppercase().to_string() } else { "".to_owned() };

        // TODO
        // Should be prefixable with "F" or "Fb"
        return write!(f, "{}{}{}", self.numeral, letter, plus);
    }
}

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
        assert!(Grade::new(6, Some('a'), false).is_some());
    }

    #[test]
    fn valid_grade_4() {
        assert!(Grade::new(7, Some('b'), true).is_some());
    }

    #[test]
    fn invalid_grade_1() {
        assert!(Grade::new(5, Some('a'), false).is_none());
    }

    #[test]
    fn invalid_grade_2() {
        assert!(Grade::new(6, None, false).is_none());
    }

    #[test]
    fn invalid_grade_3() {
        assert!(Grade::new(7, Some('d'), false).is_none());
    }

    #[test]
    fn invalid_grade_4() {
        assert!(Grade::new(10, None, false).is_none());
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
        assert_eq!(Grade::new(6, Some('a'), false).expect("Good value").to_string(), "6A");
    }

    #[test]
    fn to_string_4() {
        assert_eq!(Grade::new(7, Some('b'), true).expect("Good value").to_string(), "7B+");
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
        assert!(Grade::new(6, Some('a'), false).expect("Good value") < Grade::new(6, Some('b'), false).expect("Good value"));
    }

    #[test]
    fn lt_4() {
        assert!(Grade::new(6, Some('a'), false).expect("Good value") < Grade::new(6, Some('a'), true).expect("Good value"));
    }
}
