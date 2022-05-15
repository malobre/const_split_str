#[test]
fn split() {
    let [head, tail] = const_split_str::split!("head-tail", "-");

    assert_eq!(head, "head");
    assert_eq!(tail, "tail");
}

#[test]
fn split_once() {
    let (head, tail) = const_split_str::split_once!("head-tail", "-");

    assert_eq!(head, "head");
    assert_eq!(tail, "tail");
}
