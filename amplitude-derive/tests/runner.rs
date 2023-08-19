#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-empty_struct.rs");
    t.pass("tests/02-one_field_struct.rs");
}
