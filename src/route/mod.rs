use crate::{grades::Grade, ascent::Ascent};

/// Structure representing a particular route
pub struct Route {
    /// The route name
    pub name: Option<RouteName>,

    /// The route description
    pub description: Option<String>,

    /// The route difficulty
    pub grade: Option<Grade>,

    /// Any of the routes notable ascents
    pub notable_ascents: Option<Vec<NotableAscent>>
}

/// Enumeration of a route name. Some routes are known by one name, some by several.
pub enum RouteName {
    /// To be used when the route has one name
    Singular(String),

    /// To be used when the route has more than one name
    Multiple(Vec<String>),
}

/// Structure for a notable ascent
pub struct NotableAscent {
    /// The ascent
    pub ascent: Ascent,

    /// The reason the ascent is notable
    pub reason: NotableAscentReason
}

/// Enumeration for reasons an ascent may be notable
pub enum NotableAscentReason {
    /// One reason an ascent may be notable is if it is the first
    First,
}
