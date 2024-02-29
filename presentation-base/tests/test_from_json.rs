use std::fs;

use oca_presentation::presentation::Presentation;

#[test]
fn test_select() {
    let contents = fs::read_to_string("tests/presentation_examples/select.json")
        .expect("Should have been able to read the file");

    let pres: Result<Presentation, _> = serde_json::from_str(&contents);
    assert!(pres.is_ok());
    assert_eq!(
        contents,
        serde_json::to_string_pretty(&pres.unwrap()).unwrap()
    );
}

#[test]
fn test_signature() {
    let contents = fs::read_to_string("tests/presentation_examples/signature.json")
        .expect("Should have been able to read the file");

    let pres: Result<Presentation, _> = serde_json::from_str(&contents);
    assert!(pres.is_ok());
    assert_eq!(
        contents,
        serde_json::to_string_pretty(&pres.unwrap()).unwrap()
    );
}
