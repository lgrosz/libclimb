pub enum Modifier {
    None,
    Plus,
}

pub struct Grade {
    pub numeral: u8,
    pub letter: char,
    pub modifier: Modifier,
}

use std::fmt;

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO How do I make it so the 'f' is optional?
        write!(f, "f{}{}{}", self.numeral, self.letter, self.modifier)
    }
}

use std::str::FromStr;

impl FromStr for Modifier {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "+" => Ok(Self::Plus),
            "" => Ok(Self::None),
            _ => Err(()),
        }
    }
}
