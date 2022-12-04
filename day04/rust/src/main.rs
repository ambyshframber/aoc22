type Elf = std::ops::RangeInclusive<i32>;

fn main() {
    let input = include_str!("input.txt");
    let pairs = input.split('\n').map(parse_pair);

    let contains_count = pairs
        .clone()
        .map(does_pair_contain)
        .filter(|p| *p)
        .count();

    let overlap_count = pairs
        .map(does_pair_overlap)
        .filter(|p| *p)
        .count();

    println!("{}", contains_count);
    println!("{}", overlap_count);
}

fn parse_pair(p: &str) -> (Elf, Elf) {
    let (first, second) = p.split_once(',').unwrap();
    (parse_range(first), parse_range(second))
}

fn parse_range(r: &str) -> Elf {
    let (start, end) = r.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn does_pair_contain(pair: (Elf, Elf)) -> bool {
    let (a, b) = pair;
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn does_pair_overlap(pair: (Elf, Elf)) -> bool {
    let (a, b) = pair;
    (a.contains(b.start()) || a.contains(b.end())) || (b.contains(a.start()) || b.contains(a.end()))
}
