use super::*;

#[test]
fn no_division_and_no_plus() {
    assert!(Fontainebleau::new(1, None, false).is_ok());
}

#[test]
fn no_division_and_plus() {
    assert!(Fontainebleau::new(5, None, true).is_ok());
}

#[test]
fn division_and_no_plus() {
    assert!(Fontainebleau::new(6, Some(Division::A), false).is_ok());
}

#[test]
fn division_and_plus() {
    assert!(Fontainebleau::new(7, Some(Division::B), true).is_ok());
}

#[test]
fn level_too_low_with_division() {
    assert!(matches!(
        Fontainebleau::new(5, Some(Division::A), false).unwrap_err(),
        FontainebleauError::LevelTooLowWithDivision
    ));
}

#[test]
fn level_too_high_without_division() {
    assert!(matches!(
        Fontainebleau::new(6, None, false).unwrap_err(),
        FontainebleauError::LevelTooHighWithoutDivision
    ));
}

