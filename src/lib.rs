mod grades;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slash_v_grade() {
        let grade = grades::SlashGrade {
            upper: grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None },
            lower: grades::vermin::Grade { value: 4, modifier: grades::vermin::Modifier::None },
        };
        assert_eq!(grade.to_string(), "V4/5");
    }

    #[test]
    fn f_slash_grade() {
        let grade = grades::SlashGrade {
            upper: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::Plus },
            lower: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None },
        };
        assert_eq!(grade.to_string(), "f7a/+");
    }

    #[test]
    fn slash_b_grade() {
        let grade = grades::SlashGrade {
            upper: grades::boulder::Grade { value: grades::boulder::Value::Two },
            lower: grades::boulder::Grade { value: grades::boulder::Value::One },
        };
        assert_eq!(grade.to_string(), "B1/2");
    }
}
