#![feature(iter_array_chunks)]
#![feature(test)]

fn main() {
    let input = include_str!("input.txt").trim();

    let sum = input.split('\n').map(find_duplicate).map(priority).sum::<i32>();
    println!("duplicates sum: {}", sum);

    let badge_sum = input.split('\n').array_chunks::<3>().map(find_threeplicate).map(priority).sum::<i32>();
    println!("badge sum: {}", badge_sum);
}

fn find_duplicate(bag: &str) -> char {
    let (first, second) = bag.split_at(bag.len() / 2);
    for c in first.chars() {
        if second.contains(c) {
            return c
        }
    }
    unreachable!()
}

fn priority(c: char) -> i32 {
    (match c {
        'a'..='z' => (c as u8 - b'a') + 1,
        'A'..='Z' => (c as u8 - b'A') + 27,
        _ => unreachable!()
    }) as i32
}

fn find_threeplicate(bags: [&str; 3]) -> char {
    for c in bags[0].chars() {
        if bags[1].contains(c) && bags[2].contains(c) {
            return c
        }
    }
    unreachable!()
}

#[test]
fn duplicates() {
    assert_eq!(find_duplicate("abcABc"), 'c');
    assert_eq!(find_duplicate("abcDEfABCDeF"), 'D');
}

#[test]
fn t_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('b'), 2);

    assert_eq!(priority('C'), 26 + 3);
}

extern crate test;
use test::Bencher;
#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| main())
}
