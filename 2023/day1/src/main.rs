use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> std::io::Result<()> {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let file = File::open("./input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut sum = 0;
    for line in contents.lines() {
        let mut first_digit = None;
        let mut second_digit = 0;
        for (j, s) in line.chars().enumerate() {
            let s = s.to_string();
            for i in 1..numbers.len() {
                if line[j..].starts_with(numbers[i]) {
                    if first_digit.is_none() {
                        first_digit = Some(i);
                        second_digit = i;
                    } else {
                        second_digit = i;
                    }
                }
                match s.parse::<usize>() {
                    Ok(n) => {
                        if first_digit.is_none() {
                            first_digit = Some(n);
                            second_digit = n;
                        } else {
                            second_digit = n;
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        // println!("{line}");
        // println!("{}", 10 * first_digit.unwrap() + second_digit);
        sum += 10 * first_digit.unwrap() + second_digit;
    }
    println!("first star : didn't keep the code");
    println!("second star : {sum}");
    Ok(())
}
