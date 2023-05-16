#[test]
fn serialize() {
    let output = aseprite::Output::from("../../assets/art/character/The Convict.json").unwrap();
    println!("{output:#?}");
    assert!(output.frames.len() > 0);
}
