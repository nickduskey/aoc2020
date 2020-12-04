use crate::utils;

pub fn fetch_data() -> String {
    println!("Fetching data...");
    let res = utils::http_get("https://adventofcode.com/2020/day/1/input");
    let s = res.text();
    return s.unwrap();
}

pub fn run() {
    println!("Running part 1...");
    let data = fetch_data();
    let split = data.lines();
    let lines: Vec<&str> = split.collect();

    let mut index1: i32 = 0;
    let mut index2: i32 = 0;
    'outer: for val in lines.iter() {
        let base = lines[index1 as usize].parse::<i32>().unwrap();
        for (pos, line) in lines.iter().enumerate() {
            let i = line.parse::<i32>().unwrap();
            if pos as i32 != index1 {
                let sum = base + i;
                if sum == 2020 {
                    index2 = pos as i32;
                    break 'outer;
                }
            }
        }
        index1 += 1;
    }
    println!("Index 1 {}", index1);
    println!("Val 1 {}", lines[index1 as usize]);
    println!("Index 2 {}", index2);
    println!("Val 2 {}", lines[index2 as usize]);
    let line1 = lines[index1 as usize];
    let line2 = lines[index2 as usize];
    let num1 = line1.parse::<i32>().unwrap();
    let num2 = line2.parse::<i32>().unwrap();
    let f = num1 * num2;
    println!("f {}", f);
}