#![cfg(test)]
use rotate::rotate_string;

#[test]
fn test_rotation() {
    assert_eq!(String::from("Uryyb"), rotate_string(&String::from("Hello"), 13));
}
