use std::fs::read_to_string;

fn main() {
    // part1();
    part2();
}

#[allow(dead_code)]
fn part1() {
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

#[allow(dead_code)]
fn part2() {
    let file_path = "input2";
    let mut result = Vec::new();

    // finding the first
    for line in read_to_string(file_path).unwrap().lines() {
        println!("line: {}", &line);
        
        let mut x: i64 = -1; // 0 is a valid index
        let mut a: String = "".to_string();
        
        let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for num in numbers {
            let idx = line.find(num);
            if idx == None {
                continue;
            }
            // println!("x: {}", x);
            // println!("idx: {}", idx.unwrap());

            let g = idx.unwrap().try_into().unwrap();
            if x == -1 {
                x = g;
                a = num.to_string();
            } else if g < x {
                x = g;
                a = num.to_string();
            }
        }

        let mut j = -1;
        let mut b: char = 'a';
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                j = i.try_into().unwrap();
                b = c;
                break;
            }
        }

        let mut one_liner = "".to_string();
        if x == -1 {
            one_liner.push(b);
        } else if x < j {
            let o = string_to_numeric_char(a);
            one_liner.push(o);
        } else {
            one_liner.push(b);
        }

        // find last
        let mut jl = 0;
        let mut bl = '!';
        let revl = line.chars().rev().collect::<String>();
        for (i, c) in revl.chars().enumerate() {
            if c.is_numeric() {
                jl = i;
                bl = c;
                break;
            }
        }

        if line.rfind(bl) != None {
            jl = line.rfind(bl).unwrap();
        }

        let mut xl = 0;
        let mut al = "".to_string();
        let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for num in numbers {
            let idx = line.rfind(num);
            if idx == None {
                continue;
            }

            if xl == 0 {
                xl = idx.unwrap();
                al = num.to_string();
            } else if idx.unwrap() > xl {
                xl = idx.unwrap();
                al = num.to_string();
            }
        }


        if xl > jl {
            let o =string_to_numeric_char(al);
            if o != '!' {
                one_liner.push(o);
            }
        } else {
            one_liner.push(bl);
        }

        if one_liner.len() == 2 {
            println!("{}", one_liner);
            result.push(one_liner);
        }
    }

    let mut sum = 0;
    for r in result {
        sum += r.parse::<i32>().unwrap();
    }
    println!("sum: {:?}", sum);
}

fn string_to_numeric_char(st: String) -> char {
    let mut o = '!';
    if st == "one".to_string() {
        o = '1';
    } else if st == "two".to_string() {
        o = '2';
    } else if st == "three".to_string() {
        o = '3';
    }else if st == "four".to_string() {
        o = '4';
    }else if st == "five".to_string() {
        o = '5';
    }else if st == "six".to_string() {
        o = '6';
    }else if st == "seven".to_string() {
        o = '7';
    }else if st == "eight".to_string() {
        o = '8';
    }else if st == "nine".to_string() {
        o = '9';
    }

    return o;
}
