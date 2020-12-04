use crate::utils;

pub fn run() {
    println!("Running part 2...");
    let data = utils::fetch_data(1);
    let split = data.lines();
    let lines: Vec<&str> = split.collect();

    let mut index1: i32 = 0;
    let mut index2: i32 = 0;
    let mut index3: i32 = 0;

    'outer: for i in lines.iter() {
        let i_int = lines[index1 as usize].parse::<i32>().unwrap();
        for (j_pos, j) in lines.iter().enumerate() {
            let j_int = j.parse::<i32>().unwrap();
            for (k_pos, k) in lines.iter().enumerate() {
                let k_int = k.parse::<i32>().unwrap();
                if k_pos as i32 != index1 {
                    if k_pos != j_pos {
                        let sum = i_int + j_int + k_int;
                        if sum == 2020 {
                            index2 = j_pos as i32;
                            index3 = k_pos as i32;
                            break 'outer;
                        }
                    }
                }
            }
        }
        index1 += 1;
    }

    println!("Index 1 {}", index1);
    println!("Val 1 {}", lines[index1 as usize]);
    println!("Index 2 {}", index2);
    println!("Val 2 {}", lines[index2 as usize]);
    println!("Index 3 {}", index3);
    println!("Val 3 {}", lines[index3 as usize]);
    let line1 = lines[index1 as usize];
    let line2 = lines[index2 as usize];
    let line3 = lines[index3 as usize];
    let num1 = line1.parse::<i32>().unwrap();
    let num2 = line2.parse::<i32>().unwrap();
    let num3 = line3.parse::<i32>().unwrap();
    let f = num1 * num2 * num3;
    println!("f {}", f);
}
