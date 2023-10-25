use super::*;

#[test]
fn level_difference() {
    assert!(Hueco::new(1, None) < Hueco::new(2, None));
}

#[test]
fn minus_less_than_none() {
    assert!(Hueco::new(1, Some(Modifier::Minus)) < Hueco::new(1, None));
}

#[test]
fn minus_less_than_plus() {
    assert!(Hueco::new(1, Some(Modifier::Minus)) < Hueco::new(1, Some(Modifier::Plus)));
}

#[test]
fn none_less_than_plus() {
    assert!(Hueco::new(1, None) < Hueco::new(1, Some(Modifier::Plus)));
}

