use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(input) = line {
                let mut first: Option<char> = None;
                let mut last: Option<char> = None;
                for character in input.chars() {
                    if character.is_digit(10) {
                        if first == None {
                            first = Some(character);
                        }
                        last = Some(character);
                    }
                }
                let number: u32 = format!("{}{}", first.unwrap(), last.unwrap())
                    .parse()
                    .unwrap();
                sum += number;
            }
        }
        println!("part1 sum: {}", sum);
    }

    let mut digit_word_values = HashMap::new();
    digit_word_values.insert("one", '1');
    digit_word_values.insert("two", '2');
    digit_word_values.insert("three", '3');
    digit_word_values.insert("four", '4');
    digit_word_values.insert("five", '5');
    digit_word_values.insert("six", '6');
    digit_word_values.insert("seven", '7');
    digit_word_values.insert("eight", '8');
    digit_word_values.insert("nine", '9');

    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(input) = line {
                let mut first: Option<char> = None;
                let mut last: Option<char> = None;

                for character in input.chars().enumerate() {
                    if character.1.is_digit(10) {
                        if first == None {
                            first = Some(character.1);
                        }
                        last = Some(character.1);
                    } else if character.1.is_alphabetic() {
                        for digit_word_value in &digit_word_values {
                            if input[character.0..].starts_with(digit_word_value.0) {
                                if first == None {
                                    first = Some(*digit_word_value.1)
                                }
                                last = Some(*digit_word_value.1)
                            }
                        }
                    }
                }
                let number: u32 = format!("{}{}", first.unwrap(), last.unwrap())
                    .parse()
                    .unwrap();
                sum += number;
            }
        }
        println!("part2 sum: {}", sum);
    }
}
