fn main() {
    let input = include_str!("input.txt");

    let packet_start = find_n_unique(input.as_bytes(), 4).unwrap() + 4;
    println!("{}", packet_start);

    let msg_start = find_n_unique(input.as_bytes(), 14).unwrap() + 14;
    println!("{}", msg_start);
}

fn find_n_unique<T: Eq>(s: &[T], n: usize) -> Option<usize> {
    s.windows(n).position(|s| !slice_has_duplicates(s))
}

fn slice_has_duplicates<T: Eq>(s: &[T]) -> bool {
    s.iter().enumerate().any(|(i, v)| s[i + 1..].contains(v))
}

#[test]
fn dups() {
    assert!(slice_has_duplicates(&[1, 2, 3, 1]));
    assert!(!slice_has_duplicates(&[1, 2, 3, 4]));
}
