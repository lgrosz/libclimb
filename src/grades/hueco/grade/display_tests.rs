use super::*;

#[test]
fn level() {
    assert_eq!(Hueco::new(1, None).to_string(), "V1");
}

#[test]
fn level_plus() {
    assert_eq!(Hueco::new(1, Some(Modifier::Plus)).to_string(), "V1+");
}

#[test]
fn level_minus() {
    assert_eq!(Hueco::new(1, Some(Modifier::Minus)).to_string(), "V1-");
}
