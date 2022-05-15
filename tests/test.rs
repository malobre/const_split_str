#[test]
fn split() {
    let [head, tail] = const_split_str::split!("head-tail", "-");

    assert_eq!(head, "head");
    assert_eq!(tail, "tail");
}
