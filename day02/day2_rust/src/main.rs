fn main() {
    let input = include_str!("input.txt").trim();
    
    // part 1
    let score = input.split('\n').map(|round| {
        let (own, enemy) = parse_round(round);
        (own as i32) + 1 + (rps(own, enemy) as i32)
    }).sum::<i32>();
    println!("part 1: {}", score);

    let score = input.split('\n').map(do_round_2).sum::<i32>();
    println!("part 2: {}", score);
}

fn parse_round(r: &str) -> (RpsGuess, RpsGuess) { // self, enemy
    use RpsGuess::*;
    let (enemy, own) = r.split_once(' ').unwrap();
    let enemy = match enemy {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => unreachable!()
    };
    let own = match own {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => unreachable!()
    };
    (own, enemy)
}

fn do_round_2(r: &str) -> i32 {
    use RpsGuess::*;
    let (enemy, own) = r.split_once(' ').unwrap();

    let enemy = match enemy {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => unreachable!()
    };
    let (self_pick, result_score) = match own {
        "X" => ((enemy as i32) - 1, 0),
        "Y" => (enemy as i32, 3),
        "Z" => ((enemy as i32) + 1, 6),
        _ => unreachable!()
    };
    self_pick.rem_euclid(3) + 1 + result_score
}

#[derive(PartialEq, Copy, Clone)]
enum RpsGuess {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}
enum RpsResult {
    Win = 6,
    Loss = 0,
    Draw = 3
}

// self, enemy
fn rps(lhs: RpsGuess, rhs: RpsGuess) -> RpsResult {
    use RpsResult::*;
    let game = (lhs as i32 - rhs as i32).rem_euclid(3);
    match game {
        0 => Draw,
        1 => Win,
        2 => Loss,
        _ => unreachable!()
    }
}

fn get_loss(guess: RpsGuess) -> RpsGuess {
    use RpsGuess::*;
    match guess {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper
    }
}
