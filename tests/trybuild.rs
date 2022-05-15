#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/missing.rs");
    t.pass("tests/ui/contained.rs");
}
