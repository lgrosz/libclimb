use super::*;

#[test]
fn level_difference() {
    assert!(Fontainebleau::new(1, None, false).expect("Good value") < Fontainebleau::new(2, None, false).expect("Good value"));
}

#[test]
fn plus_difference_without_division() {
    assert!(Fontainebleau::new(1, None, false).expect("Good value") < Fontainebleau::new(1, None, true).expect("Good value"));
}

#[test]
fn division_difference() {
    assert!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value") < Fontainebleau::new(6, Some(Division::B), false).expect("Good value"));
}

#[test]
fn plus_different_with_division() {
    assert!(Fontainebleau::new(6, Some(Division::A), false).expect("Good value") < Fontainebleau::new(6, Some(Division::A), true).expect("Good value"));
}

