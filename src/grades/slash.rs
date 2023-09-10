pub struct SlashGrade<T> {
    pub lower: T,
    pub upper: T,
}

use std::fmt;

impl<T> fmt::Display for SlashGrade<T>
where 
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let lower_string = self.lower.to_string();
        let mut upper_string = self.upper.to_string();

        // This is a little ugly, but it is essentially stripping the beginning of `upper_string`
        // until it shares no characters with `lower_string`
        for c in lower_string.chars() {
            if upper_string.starts_with(c) {
                let mut upper_string_chars = upper_string.chars();
                upper_string_chars.next();
                upper_string = upper_string_chars.as_str().to_string();
            } else {
                break;
            }
        }

        write!(f, "{}/{}", lower_string, upper_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grades::vermin;
    use crate::grades::fontainebleau;
    use crate::grades::boulder;

    #[test]
    fn slash_v_grade() {
        let grade = SlashGrade {
            upper: vermin::Grade { value: 5, modifier: vermin::Modifier::None },
            lower: vermin::Grade { value: 4, modifier: vermin::Modifier::None },
        };
        assert_eq!(grade.to_string(), "V4/5");
    }

    #[test]
    fn f_slash_grade() {
        let grade = SlashGrade {
            upper: fontainebleau::Grade { numeral: 7, letter: 'a', modifier: fontainebleau::Modifier::Plus },
            lower: fontainebleau::Grade { numeral: 7, letter: 'a', modifier: fontainebleau::Modifier::None },
        };
        assert_eq!(grade.to_string(), "f7a/+");
    }

    #[test]
    fn slash_b_grade() {
        let grade = SlashGrade {
            upper: boulder::Grade { value: boulder::Value::Two },
            lower: boulder::Grade { value: boulder::Value::One },
        };
        assert_eq!(grade.to_string(), "B1/2");
    }
}
