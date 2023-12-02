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
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    if let Ok(lines) = read_lines("./input.txt") {
        let mut part1sum = 0;
        let mut part2sum = 0;
        for line in lines {
            match line {
                Ok(line) => {
                    let mut game_red_max = 0;
                    let mut game_green_max = 0;
                    let mut game_blue_max = 0;
                    let mut count_part_1 = true;
                    let mut split_line = line.split(':');
                    let game_id: u32 = split_line
                        .nth(0)
                        .unwrap_or_default()
                        .split_ascii_whitespace()
                        .nth(1)
                        .unwrap_or_default()
                        .parse()
                        .unwrap_or_default();
                    for hand in split_line.nth(0).unwrap_or_default().split(";") {
                        for draw in hand.split(',') {
                            match &draw.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                                &[number_str, color, ..] => {
                                    let number: u32 = number_str.parse().unwrap_or_default();
                                    match color {
                                        "red" if number > game_red_max => game_red_max = number,
                                        "green" if number > game_green_max => {
                                            game_green_max = number
                                        }
                                        "blue" if number > game_blue_max => game_blue_max = number,
                                        _ => (),
                                    }
                                    match color {
                                        "red" if number > red_max => count_part_1 = false,
                                        "green" if number > green_max => count_part_1 = false,
                                        "blue" if number > blue_max => count_part_1 = false,
                                        _ => (),
                                    }
                                }
                                _ => print!("Error"),
                            }
                        }
                    }
                    let power = game_red_max * game_green_max * game_blue_max;
                    println!("{}, {} - {}", game_id, power, line);

                    if count_part_1 {
                        part1sum += game_id;
                    }
                    part2sum += power;
                }
                _ => println!("Error in file"),
            }
        }
        println!("part1 sum: {}", part1sum);
        println!("part2 sum: {}", part2sum);
    }
}
