use super::*;

#[test]
fn invalid_str() {
    assert!(matches!(
        "".parse::<Hueco>().unwrap_err(),
        HuecoParseError::InvalidStr
    ));
}

#[test]
fn no_modifier() {
    assert_eq!("V1".parse::<Hueco>().expect("Good value"), Hueco::new(1, None))
}

#[test]
fn plus_modifier() {
    assert_eq!("V1+".parse::<Hueco>().expect("Good value"), Hueco::new(1, Some(Modifier::Plus)));
}

#[test]
fn minus_modifier() {
    assert_eq!("V1-".parse::<Hueco>().expect("Good value"), Hueco::new(1, Some(Modifier::Minus)));
}
