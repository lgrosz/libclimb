pub mod vermin;
pub mod fontainebleau;
pub mod boulder;

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
