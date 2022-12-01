fn main() {
    let input = include_str!("input.txt");
    let mut elves =
        input.split("\n\n").map(|elf| {
            elf.trim().split('\n')
               .map(|item| item.parse::<i32>().unwrap())
               .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    elves.sort_by(|a, b| b.cmp(a));
    let top_3 = elves[0..3].iter().inspect(|e| println!("{}", e)).sum::<i32>();
    println!("{}", top_3)
}
