// TODO vermin-->hueco?
pub enum Modifier {
    None,
    Plus,
    Minus,
}

pub struct Grade {
    pub value: u8,
    pub modifier: Modifier,
}

use std::fmt;

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::Minus => write!(f, "-"),
            Modifier::None => write!(f, ""),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V{}{}", self.value, self.modifier)
    }
}

use std::str::FromStr;

impl FromStr for Modifier {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "-" => Ok(Self::Minus),
            "+" => Ok(Self::Plus),
            "" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

// TODO
//impl FromStr for Grade {
//    type Err = ();
//    
//    fn from_str(s: &str) -> Result<Self, ()> {
//        // TODO
//    }
//}
