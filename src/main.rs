#![allow(dead_code)]

mod grades {

    pub mod vermin {
        #[derive(Debug)]
        pub enum Modifier {
            None,
            Plus,
            Minus,
        }

        #[derive(Debug)]
        pub struct Grade {
            pub value: u8,
            pub modifier: Modifier,
        }
    }

    pub mod fontainebleau {
        #[derive(Debug)]
        pub enum Modifier {
            None,
            Plus,
        }

        #[derive(Debug)]
        pub struct Grade {
            pub numeral: u8,
            pub letter: char,
            pub modifier: Modifier,
        }
    }

    pub mod boulder {
        #[derive(Debug)]
        pub enum Value {
            One,
            Two,
            Three,
        }

        #[derive(Debug)]
        pub struct Grade {
            pub value: Value,
            pub year: u16,
        }
    }

    #[derive(Debug)]
    pub struct SlashGrade<T> {
        pub upper: T,
        pub lower: T,
    }
}


use std::fmt;

impl fmt::Display for grades::vermin::Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            grades::vermin::Modifier::Plus => write!(f, "+"),
            grades::vermin::Modifier::Minus => write!(f, "-"),
            grades::vermin::Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for grades::vermin::Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V{}{}", self.value, self.modifier)
    }
}

impl fmt::Display for grades::fontainebleau::Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            grades::fontainebleau::Modifier::Plus => write!(f, "+"),
            grades::fontainebleau::Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for grades::fontainebleau::Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO How do I make it so the 'f' is optional?
        write!(f, "f{}{}{}", self.numeral, self.letter, self.modifier)
    }
}

impl fmt::Display for grades::boulder::Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            grades::boulder::Value::One => write!(f, "1"),
            grades::boulder::Value::Two => write!(f, "2"),
            grades::boulder::Value::Three => write!(f, "3"),
        }
    }
}

impl fmt::Display for grades::boulder::Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B{}", self.value)
    }
}

impl<T> fmt::Display for grades::SlashGrade<T>
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

fn main() {
    let v_grade = grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None };
    println!("{}", v_grade);

    let v_grade_with_modifier = grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::Plus };
    println!("{}", v_grade_with_modifier);

    let slash_v_grade = grades::SlashGrade {
        upper: grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None },
        lower: grades::vermin::Grade { value: 4, modifier: grades::vermin::Modifier::None },
    };
    println!("{}", slash_v_grade);

    let f_grade = grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None };
    println!("{}", f_grade);

    let slash_f_grade = grades::SlashGrade {
        upper: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::Plus },
        lower: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None },
    };
    println!("{}", slash_f_grade);

    let b_grade = grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 };
    println!("{}", b_grade);

    let slash_b_grade = grades::SlashGrade {
        upper: grades::boulder::Grade { value: grades::boulder::Value::Two, year: 1960 },
        lower: grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 },
    };
    println!("{}", slash_b_grade);
}
