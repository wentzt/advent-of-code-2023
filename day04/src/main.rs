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
        let mut part1sum = 0;
        let part2sum: usize;
        let mut lines_wins: Vec<usize> = Vec::new();
        for line in lines.map(Result::unwrap) {
            let mut line_value: Option<u32> = None;
            let mut line_wins = 0;
            let split_line = line.split('|').collect::<Vec<_>>();
            let winning_numbers_str = split_line[0].split(':').nth(1).unwrap();
            let numbers_i_have_str = split_line[1];
            let winning_numbers = winning_numbers_str
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let numbers_i_have = numbers_i_have_str
                .split_ascii_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            for winning_number in winning_numbers {
                if numbers_i_have.contains(&winning_number) {
                    line_value = match line_value {
                        None => Some(1),
                        Some(value) => Some(value * 2),
                    };
                    line_wins += 1;
                }
            }
            if let Some(value) = line_value {
                part1sum += value;
            }
            lines_wins.push(line_wins);
        }

        let mut scratchcards: Vec<_> = lines_wins.iter().enumerate().collect();
        let mut card_index = 0;
        while card_index < scratchcards.len() {
            let (current_card_index, wins) = scratchcards[card_index];
            if *wins > 0 {
                let cards_to_add: Vec<_> = scratchcards
                    [current_card_index + 1..current_card_index + 1 + *wins]
                    .iter()
                    .map(|c| c.to_owned())
                    .collect();
                cards_to_add.iter().for_each(|c| scratchcards.push(*c))
            }
            card_index += 1;
        }
        part2sum = scratchcards.len();

        println!("part1 sum: {}", part1sum);
        println!("part2 sum: {}", part2sum);
    }
}

// Recursive route didn't work the way I thought
// fn get_scorecard_count(card_wins: &Vec<usize>, indent: u32) -> u32 {
//     let mut count = 1; // Self
//     if let Some((&head, tail)) = card_wins.split_first() {
//         if head > 0 {
//             let len = head.min(tail.len());
//             for idx in 0..len {
//                 println!("{} - {}, {:?}", indent, len, &tail[idx..len]);
//                 let copies = get_scorecard_count(&tail[idx..len].into(), indent + 1);
//                 //println!("{} - {}, {}, {:?}", indent, copies, len, &tail[idx..len]);
//                 count += copies;
//             }
//         }
//     }
//     count
// }
