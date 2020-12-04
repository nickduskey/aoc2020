use crate::day2::part1;

pub fn run() {
    println!("Running part 2...");
    let data = part1::fetch_data();
    let split = data.lines();
    let lines: Vec<&str> = split.collect();

    let mut num_valid_password: i32 = 0;
    for val in lines.iter() {
        let pieces: Vec<String> = val.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let positions: Vec<i32> = pieces[0]
            .split("-")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        // Account for the 1 index instead of 0 index
        let pos1: i32 = positions[0].clone() - 1;
        let pos2: i32 = positions[1].clone() - 1;

        let letter_to_check: String = pieces[1]
            .split(":")
            .collect();

        let password: String = pieces[2].clone();

        // check first position
        let first_pos = password.chars()
            .nth(pos1 as usize)
            .unwrap()
            .to_string();
        // check second position
        let second_pos = password.chars()
            .nth(pos2 as usize)
            .unwrap()
            .to_string();
        println!("=======================================");
        println!("pos1: {}", pos1);
        println!("pos2: {}", pos2);
        println!("password: {}", password);
        println!("letter_to_check: {}", letter_to_check);
        println!("first_pos: {}", first_pos);
        println!("second_pos: {}", second_pos);
        if first_pos == letter_to_check && second_pos != letter_to_check {
            println!("first_pos == letter_to_check, is valid");
            num_valid_password += 1;
        }
        if first_pos != letter_to_check && second_pos == letter_to_check {
            println!("second_pos == letter_to_check, is valid");
            num_valid_password += 1;
        }
    }
    println!("num_valid_passwords: {}", num_valid_password);
}