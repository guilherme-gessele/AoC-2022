use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let elfs: Vec<&str> = input.split("\n\n").collect();

    let mut calories: Vec<u32> = Vec::new();

    for elf in elfs {
        let amount: u32 = elf.split("\n").map(|cal| cal.parse::<u32>().unwrap()).sum();

        calories.push(amount);
    }

    if let Some(max_calories) = calories.iter().max() {
        println!("Most calories carried: {max_calories}");
    }

    calories.sort_by(|a, b| b.cmp(a));
    let total: u32 = calories[0..3].iter().sum();
    println!("The top three elves are carrying {total} calories");
}
