#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-empty_struct.rs");
    t.pass("tests/02-one_field_struct.rs");
    t.pass("tests/03-basic_enum.rs");
    t.pass("tests/04-mixed_enum.rs");
    t.pass("tests/05-struct_rename_attr.rs");
}
