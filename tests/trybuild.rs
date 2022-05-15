#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/missing.rs");
    t.compile_fail("tests/ui/missing_once.rs");
    t.pass("tests/ui/contained.rs");
    t.pass("tests/ui/contained_once.rs");
}
