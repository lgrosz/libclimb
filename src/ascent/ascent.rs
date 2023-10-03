/// Structure representing an ascent
#[derive(PartialEq, PartialOrd)]
pub struct Ascent {
    /// Party responsible for the ascent
    pub party: Option<Party>,

    /// Temporal unit which the ascent occured
    pub time_point: Option<TimePoint>
}

/// Either one or more people
#[derive(PartialEq, PartialOrd)]
pub enum Party {
    /// Single person party
    Person(Person),

    /// Multiple person party
    People(Vec<Person>),
}

/// Representing a person
#[derive(PartialEq, PartialOrd)]
pub struct Person {
    /// Person's first name
    pub first_name: String,

    /// Person's last name
    pub last_name: String,
}

/// Temporal unit meant to describe a point in time
#[derive(PartialEq, PartialOrd)]
pub enum TimePoint {
    /// Specifies some year
    Year(u16),

    /// Specifies some season
    Season(Season),

    /// Specifies some month
    Month(chrono::Month),

    /// Specifies some date
    Date(chrono::NaiveDate)
}

/// Enumeration of the four common seasons of the year
#[derive(PartialEq, PartialOrd)]
pub enum Season {
    /// Spring
    Spring,

    /// Summer
    Summer,

    /// Fall or Autumn
    Fall,

    /// Winter
    Winter
}
