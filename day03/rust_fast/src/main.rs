#![feature(iter_array_chunks)]
#![feature(test)]

fn main() {
    let input = include_str!("input.txt");
    let (duplicate_sum, badge_sum) = input.split('\n')
        .map(pass_1)
        .array_chunks::<3>()
        .map(pass_2)
        .fold((0, 0), |(acc_dup, acc_badge), (dup, badge)| (acc_dup + dup, acc_badge + badge));

    println!("duplicate sum: {}", duplicate_sum);
    println!("badge sum: {}", badge_sum);
}

fn pass_1(bag: &str) -> (u32, u64) {
    let (first, second) = bag.split_at(bag.len() / 2);
    let efirst = encode(first);
    let esecond = encode(second);
    let overlap = efirst & esecond;
    let union = efirst | esecond;
    (overlap.trailing_zeros(), union)
}

fn pass_2(group: [(u32, u64); 3]) -> (u32, u32) {
    let dup_sum = group.iter().map(|e| e.0).sum::<u32>();
    let badge_ovelap = group.iter().fold(-1i64 as u64, |acc, elf| acc & elf.1);
    let badge_priority = badge_ovelap.trailing_zeros();
    (dup_sum, badge_priority)
}

fn encode(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| {
        acc | 1 << priority(c)
    })
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8 - b'a') + 1,
        'A'..='Z' => (c as u8 - b'A') + 27,
        _ => unreachable!()
    }
}

extern crate test;
#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(main)
}
