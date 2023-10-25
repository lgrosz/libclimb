use super::*;

#[test]
fn invalid_str() {
    assert!(matches!(
        "".parse::<Fontainebleau>().unwrap_err(),
        FontainebleauParseError::InvalidStr
    ));
}

#[test]
fn level_too_low_with_division() {
    assert!(matches!(
        "5B+".parse::<Fontainebleau>().unwrap_err(),
        FontainebleauParseError::BadGrade(FontainebleauError::LevelTooLowWithDivision)
    ));
}

#[test]
fn level_too_high_without_division() {
    assert!(matches!(
        "6".parse::<Fontainebleau>().unwrap_err(),
        FontainebleauParseError::BadGrade(FontainebleauError::LevelTooHighWithoutDivision)
    ));
}

#[test]
fn full() {
    assert_eq!("F8A+".parse::<Fontainebleau>().expect("Good value"), Fontainebleau::new(8, Some(Division::A), true).expect("Good value"))
}

#[test]
fn no_prefix_with_plus() {
    assert_eq!("8A+".parse::<Fontainebleau>().expect("Good value"), Fontainebleau::new(8, Some(Division::A), true).expect("Good value"))
}

#[test]
fn no_prefix_without_plus() {
    assert_eq!("8A".parse::<Fontainebleau>().expect("Good value"), Fontainebleau::new(8, Some(Division::A), false).expect("Good value"))
}

#[test]
fn prefix_without_plus() {
    assert_eq!("F8A".parse::<Fontainebleau>().expect("Good value"), Fontainebleau::new(8, Some(Division::A), false).expect("Good value"))
}

