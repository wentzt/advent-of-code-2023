use std::{
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
        let all_lines: Vec<String> = lines.map(Result::unwrap).collect();
        let mut part1sum = 0;
        let mut part2sum = 0;
        let mut gears: Vec<(usize, usize, u32, Option<u32>)> = Vec::new();
        for (line_index, line) in all_lines.iter().enumerate() {
            let numbers = find_numbers(&line);
            for number in numbers {
                if line_index > 0 {
                    if let Some(gear_index) =
                        all_lines[line_index - 1][number.0..=number.1].find('*')
                    {
                        let gear_line_index = number.0 + gear_index;
                        //print!("{} ", gear_line_index);
                        if let Some(gear) = gears
                            .iter_mut()
                            .find(|g| g.0 == line_index - 1 && g.1 == gear_line_index)
                        {
                            gear.3 = Some(number.2);
                        } else {
                            gears.push((line_index - 1, gear_line_index, number.2, None));
                        }
                    }
                    if all_lines[line_index - 1][number.0..=number.1]
                        .contains(|c: char| !c.is_ascii_digit() && c != '.')
                    {
                        part1sum += number.2;
                        //print!("{} ", number.2);
                        continue;
                    }
                }
                if let Some(gear_index) = line[number.0..=number.1].find('*') {
                    let gear_line_index = number.0 + gear_index;
                    //print!("{} ", gear_line_index);
                    if let Some(gear) = gears
                        .iter_mut()
                        .find(|g| g.0 == line_index && g.1 == gear_line_index)
                    {
                        gear.3 = Some(number.2);
                    } else {
                        gears.push((line_index, gear_line_index, number.2, None));
                    }
                }
                if line[number.0..=number.1].contains(|c: char| !c.is_ascii_digit() && c != '.') {
                    part1sum += number.2;
                    //print!("{} ", number.2);
                    continue;
                }
                if line_index < all_lines.len() - 1 {
                    if let Some(gear_index) =
                        all_lines[line_index + 1][number.0..=number.1].find('*')
                    {
                        let gear_line_index = number.0 + gear_index;
                        //print!("{} ", gear_line_index);
                        if let Some(gear) = gears
                            .iter_mut()
                            .find(|g| g.0 == line_index + 1 && g.1 == gear_line_index)
                        {
                            gear.3 = Some(number.2);
                        } else {
                            gears.push((line_index + 1, gear_line_index, number.2, None));
                        }
                    }
                    if all_lines[line_index + 1][number.0..=number.1]
                        .contains(|c: char| !c.is_ascii_digit() && c != '.')
                    {
                        part1sum += number.2;
                        //print!("{} ", number.2);
                        continue;
                    }
                }
            }
        }
        for potential_gear in gears.iter() {
            if let Some(n2) = potential_gear.3 {
                println!(
                    "line: {}, index: {}, n1: {}, n2: {}",
                    potential_gear.0, potential_gear.1, potential_gear.2, n2
                );
                part2sum += potential_gear.2 * n2;
            }
        }
        println!("part1 sum: {}", part1sum);
        println!("part2 sum: {}", part2sum);
    }
}

fn find_numbers(str: &String) -> Vec<(usize, usize, u32)> {
    let mut indexes: Vec<(usize, usize, u32)> = Vec::new();
    let mut current_index: Option<usize> = None;
    let mut current_digits = "".to_owned();
    for (index, c) in str.chars().enumerate() {
        if c.is_ascii_digit() {
            if current_index == None && index == 0 {
                current_index = Some(0);
            } else if current_index == None && index > 0 {
                current_index = Some(index - 1);
            }

            current_digits.push(c);
        } else {
            if let Some(current_index) = current_index {
                indexes.push((current_index, index, current_digits.parse::<u32>().unwrap()))
            }
            current_index = None;
            current_digits.clear();
        }
    }
    if let Some(current_index) = current_index {
        indexes.push((
            current_index,
            str.len() - 1,
            current_digits.parse::<u32>().unwrap(),
        ))
    }
    indexes
}
