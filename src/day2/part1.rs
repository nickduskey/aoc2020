use crate::utils;

pub fn fetch_data() -> String {
    println!("Fetching data...");
    return utils::fetch_data(2);
}

pub fn run() {
    println!("Running part 1...");
    let data = fetch_data();
    let split = data.lines();
    let lines: Vec<&str> = split.collect();

    let mut num_valid_passwords: i32 = 0;
    for val in lines.iter() {
        let pieces: Vec<String> = val.split_whitespace().map(|s| s.to_string()).collect();

        // get min and max occurrences
        let min_max_pieces: Vec<i32> = pieces[0]
            .split("-")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let min: i32 = min_max_pieces[0].clone();
        let max: i32 = min_max_pieces[1].clone();

        // get letter
        let letter_to_check: String = pieces[1]
            .split(":")
            .collect();

        // get password
        let password = pieces[2].clone();

        // count instances of letter in string
        let count: i32 = password.matches(&letter_to_check).count() as i32;

        // add to valid count if valid
        if count >= min && count <= max {
            println!("Is valid.");
            num_valid_passwords += 1;
        }
    }
    println!("Total valid passwords: {}", num_valid_passwords);
}