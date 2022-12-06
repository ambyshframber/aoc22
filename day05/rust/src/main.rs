fn main() {
    let stacks_init = include_str!("stacks.txt");
    let mut stacks = Stacks::new(stacks_init);
    
    let moves = include_str!("moves.txt").split('\n').map(|m| m.parse::<Move>().unwrap());
    stacks.do_moves(moves);

    println!("part 1: {}", stacks.tops_p1());
    println!("part 2: {}", stacks.tops_p2())
}

#[derive(Debug)]
struct Stacks {
    stacks: [Vec<char>; 9],
    stacks_part2: [Vec<char>; 9]
}
impl Stacks {
    fn new(init: &str) -> Stacks {
        let mut stacks: [Vec<char>; 9] = Default::default();
        for row in init.split('\n').rev().skip(1) { // up from bottom of stacks, skip line with stack numbers
            for i in 0..9 {
                let crate_ = row.as_bytes()[(i * 4) + 1] as char;
                if crate_ != ' ' {
                    stacks[i].push(crate_)
                }
            }
        }
        Stacks { stacks_part2: stacks.clone(), stacks }
    }

    fn do_move_v1(&mut self, mv: Move) {
        let mut src = Vec::new();
        std::mem::swap(&mut src, &mut self.stacks[mv.from]); // keep the borrow checker happy
        let dest = &mut self.stacks[mv.to];
        dest.extend(&mut src.pop_multiple(mv.count));
        std::mem::swap(&mut src, &mut self.stacks[mv.from]);
    }
    fn do_move_v2(&mut self, mv: Move) {
        let mut crates = Vec::with_capacity(mv.count);
        crates.extend(self.stacks_part2[mv.from].pop_multiple(mv.count));
        self.stacks_part2[mv.to].extend(crates.iter().rev());
    }

    fn do_move(&mut self, mv: Move) {
        self.do_move_v1(mv);
        self.do_move_v2(mv)
    }
    fn do_moves<I>(&mut self, mvs: I)
    where I: Iterator<Item = Move> {
        for mv in mvs {
            self.do_move(mv)
        }
    }

    fn tops_p1(&self) -> String {
        self.stacks.iter().map(|s| s[s.len() - 1]).collect()
    }
    fn tops_p2(&self) -> String {
        self.stacks_part2.iter().map(|s| s[s.len() - 1]).collect()
    }
}

#[derive(Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}
impl std::str::FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let count = parts.nth(1).unwrap().parse().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1; // correct off-by-one error
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Ok(Move { count, from, to })
    }
}

trait CrateStack<T> {
    fn pop_multiple(&mut self, count: usize) -> StackPop<T>;
    fn take_multiple(&mut self, count: usize) -> Vec<T>;
}
impl<T: Clone> CrateStack<T> for Vec<T> {
    fn pop_multiple(&mut self, count: usize) -> StackPop<T> {
        StackPop { stack: self, count }
    }
    fn take_multiple(&mut self, count: usize) -> Vec<T> {
        Vec::from(&self[self.len() - count..])
    }
}

#[test]
fn take_multiple() {
    let mut v = vec![1, 2, 3, 4];
    assert_eq!(
        v.take_multiple(2),
        vec![3, 4]
    );
    assert_eq!(v, vec![1, 2])
}

struct StackPop<'a, T> {
    stack: &'a mut Vec<T>,
    count: usize
}
impl<T> Iterator for StackPop<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            self.stack.pop()
        }
        else {
            None
        }
    }
}

