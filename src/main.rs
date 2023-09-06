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



fn main() {
    let v_grade = grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None };
    println!("{:?}", v_grade);

    let slash_v_grade = grades::SlashGrade {
        upper: grades::vermin::Grade { value: 5, modifier: grades::vermin::Modifier::None },
        lower: grades::vermin::Grade { value: 4, modifier: grades::vermin::Modifier::None },
    };
    println!("{:?}", slash_v_grade);

    let f_grade = grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None };
    println!("{:?}", f_grade);

    let slash_f_grade = grades::SlashGrade {
        upper: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::Plus },
        lower: grades::fontainebleau::Grade { numeral: 7, letter: 'a', modifier: grades::fontainebleau::Modifier::None },
    };
    println!("{:?}", slash_f_grade);

    let b_grade = grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 };
    println!("{:?}", b_grade);

    let slash_b_grade = grades::SlashGrade {
        upper: grades::boulder::Grade { value: grades::boulder::Value::One, year: 1960 },
        lower: grades::boulder::Grade { value: grades::boulder::Value::Two, year: 1960 },
    };
    println!("{:?}", slash_b_grade);
}
