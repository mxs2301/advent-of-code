use regex::Regex;
use std::{fs::File, io::*};

fn main() -> Result<()> {
    println!("Hello, world!");
    let expression = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|)").unwrap();

    let path = String::from("text.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut first: char = '0';
    let mut last: char = '0';
    let mut result: u64 = 0;
    let mut first_num: bool = true;

    let mut numbers = String::new();

    for line in reader.lines() {
        for x in line.unwrap().chars() {
            if x.is_numeric() && first_num {
                first = x;
                first_num = false;
                last = x;
            } else if x.is_numeric() {
                last = x;
            }
        }

        first_num = true;
        numbers.push(first);
        numbers.push(last);
        let char_to_num: u64 = numbers.parse().unwrap();
        numbers.clear();
        result = result + char_to_num;
    }
    println!("{}", result);

    let a = String::from("two1seven");
    Ok(())
}

fn is_number_word(num: String) -> u64 {
    match num.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
