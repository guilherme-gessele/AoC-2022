use std::{error::Error, fs};

use futures::executor::block_on;

pub async fn play(input: &Vec<&str>, strategy: impl Fn(&str) -> u32) -> u32 {
    input.into_iter().map(|round| strategy(round)).sum()
}

pub fn first_strategy(round: &str) -> u32 {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

pub fn second_strategy(round: &str) -> u32 {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

async fn async_main(rounds: &Vec<&str>) {
    let first_strategy = play(&rounds, first_strategy);
    let second_strategy = play(&rounds, second_strategy);

    let (first_score, second_score) = futures::join!(first_strategy, second_strategy);

    println!(
        "The total score following the first strategy is {}",
        first_score
    );

    println!(
        "The total score following the second strategy is {}",
        second_score
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let rounds: Vec<&str> = input.lines().collect();

    block_on(async_main(&rounds));

    Ok(())
}

#[cfg(test)]
mod test {

    #[cfg(test)]
    mod first_strategy {

        use crate::first_strategy;

        #[test]
        fn rock_vs_rock() {
            assert_eq!(first_strategy("A X"), 4);
        }

        #[test]
        fn rock_vs_paper() {
            assert_eq!(first_strategy("A Y"), 8);
        }

        #[test]
        fn rock_vs_scissors() {
            assert_eq!(first_strategy("A Z"), 3);
        }

        #[test]
        fn paper_vs_rock() {
            assert_eq!(first_strategy("B X"), 1);
        }

        #[test]
        fn paper_vs_paper() {
            assert_eq!(first_strategy("B Y"), 5);
        }

        #[test]
        fn paper_vs_scissors() {
            assert_eq!(first_strategy("B Z"), 9);
        }

        #[test]
        fn scissors_vs_rock() {
            assert_eq!(first_strategy("C X"), 7);
        }

        #[test]
        fn scissors_vs_paper() {
            assert_eq!(first_strategy("C Y"), 2);
        }

        #[test]
        fn scissors_vs_scissors() {
            assert_eq!(first_strategy("C Z"), 6);
        }
    }

    mod second_strategy {

        use crate::second_strategy;

        #[test]
        fn rock_and_lose() {
            assert_eq!(second_strategy("A X"), 3);
        }

        #[test]
        fn rock_and_draw() {
            assert_eq!(second_strategy("A Y"), 4);
        }

        #[test]
        fn rock_and_win() {
            assert_eq!(second_strategy("A Z"), 8);
        }

        #[test]
        fn paper_and_lose() {
            assert_eq!(second_strategy("B X"), 1);
        }

        #[test]
        fn paper_and_draw() {
            assert_eq!(second_strategy("B Y"), 5);
        }

        #[test]
        fn paper_and_win() {
            assert_eq!(second_strategy("B Z"), 9);
        }

        #[test]
        fn scissors_and_lose() {
            assert_eq!(second_strategy("C X"), 2);
        }

        #[test]
        fn scissors_and_draw() {
            assert_eq!(second_strategy("C Y"), 6);
        }

        #[test]
        fn scissors_and_win() {
            assert_eq!(second_strategy("C Z"), 7);
        }
    }
}
