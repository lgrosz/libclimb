use super::*;

#[test]
fn level() {
    assert_eq!(Fontainebleau::new(1, None, false).expect("Good value").to_string(), "1");
}

#[test]
fn level_plus() {
    assert_eq!(Fontainebleau::new(1, None, true).expect("Good value").to_string(), "1+");
}

#[test]
fn level_division() {
    assert_eq!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value").to_string(), "6A");
}

#[test]
fn level_division_plus() {
    assert_eq!(Fontainebleau::new(7, Some(Division::B), true).expect("Good value").to_string(), "7B+");
}
