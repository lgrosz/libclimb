mod grades;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v_grade_to_string() {
        let grade = grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None };
        assert_eq!(grade.to_string(), "V5");
    }

    #[test]
    fn v_grade_with_modifier_to_string() {
        let grade = grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::Plus };
        assert_eq!(grade.to_string(), "V5+");
    }

    #[test]
    fn slash_v_grade() {
        let grade = grades::SlashGrade {
            upper: grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None },
            lower: grades::vermin::Grade { value: 4, modifier: grades::vermin::Modifier::None },
        };
        assert_eq!(grade.to_string(), "V4/5");
    }

    #[test]
    fn f_grade() {
        let grade = grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None };
        assert_eq!(grade.to_string(), "f7a");
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
    fn b_grade() {
        let grade = grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 };
        assert_eq!(grade.to_string(), "B1");
    }


    #[test]
    fn slash_b_grade() {
        let grade = grades::SlashGrade {
            upper: grades::boulder::Grade { value: grades::boulder::Value::Two, year: 1960 },
            lower: grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 },
        };
        assert_eq!(grade.to_string(), "B1/2");
    }
}
