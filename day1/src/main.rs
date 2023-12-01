use std::fs::read_to_string;

fn main() {
    let file_path = "input";
    let mut result = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        let mut line_nums = String::from("");
        let mut digit_count = 0;
        for (_i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                line_nums.push(c);
                digit_count += 1;
                break;
            }
        }

        let l = line.chars().rev().collect::<String>();
        for (_i, c) in l.chars().enumerate() {
            if c.is_numeric() && digit_count == 1 {
                line_nums.push(c);
                break;
            }
        }

        if line_nums.len() == 2 {
            result.push(line_nums);
        }
    }

    let mut sum = 0;
    for r in result {
        sum += r.parse::<i32>().unwrap();
    }
    println!("{:?}", sum);
}
