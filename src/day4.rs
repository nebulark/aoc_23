use std::collections::VecDeque;

pub fn part1() {
    let mut winners = Vec::new();
    let mut numbers = Vec::new();

    let mut sum = 0;

    for (line_idx, line) in std::fs::read_to_string("inputs/input_4.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        // Card   1: 69 24 51 87  9 49 17 16 21 48 |  5 52 86 35 57 18 60 84 50 76 96 47 38 41 34 36 55 20 25 37  6 70 66 45  3
        let shortend = line.split_once(':').unwrap().1.trim();
        // 69 24 51 87  9 49 17 16 21 48 |  5 52 86 35 57 18 60 84 50 76 96 47 38 41 34 36 55 20 25 37  6 70 66 45  3

        let (winner_str, number_str) = shortend.split_once('|').unwrap();

        fn parse(s: &str, v: &mut Vec<i32>) {
            v.clear();
            v.extend(
                s.split_ascii_whitespace()
                    .into_iter()
                    .map(|e| e.parse::<i32>().unwrap()),
            )
        }

        parse(winner_str, &mut winners);
        parse(number_str, &mut numbers);

        winners.sort();
        numbers.sort();

        let mut winner_it = winners.iter().peekable();
        let mut numbers_it = numbers.iter().peekable();

        let mut matches = 0;
        while let (Some(&winner), Some(&number)) = (winner_it.peek(), numbers_it.peek()) {
            if winner == number {
                matches += 1;
                numbers_it.next();
                winner_it.next();
            } else if number < winner {
                numbers_it.next();
            } else {
                winner_it.next();
            }
        }
        if matches > 0 {
            sum += 2u32.pow(matches - 1);
            println!("line {} - matches {}", line_idx, matches);
        }
    }

    println!("day 4 part 1: {}", sum);
}

pub fn part2() {
    let mut winners = Vec::new();
    let mut numbers = Vec::new();

    let mut total_cards = 0;
    let mut next_extra_copies = VecDeque::new();

    for line in std::fs::read_to_string("inputs/input_4.txt")
        .unwrap()
        .lines()
    {
        // Card   1: 69 24 51 87  9 49 17 16 21 48 |  5 52 86 35 57 18 60 84 50 76 96 47 38 41 34 36 55 20 25 37  6 70 66 45  3
        let shortend = line.split_once(':').unwrap().1.trim();
        // 69 24 51 87  9 49 17 16 21 48 |  5 52 86 35 57 18 60 84 50 76 96 47 38 41 34 36 55 20 25 37  6 70 66 45  3

        let (winner_str, number_str) = shortend.split_once('|').unwrap();

        fn parse(s: &str, v: &mut Vec<i32>) {
            v.clear();
            v.extend(
                s.split_ascii_whitespace()
                    .into_iter()
                    .map(|e| e.parse::<i32>().unwrap()),
            )
        }

        parse(winner_str, &mut winners);
        parse(number_str, &mut numbers);

        winners.sort();
        numbers.sort();

        let mut winner_it = winners.iter().peekable();
        let mut numbers_it = numbers.iter().peekable();

        let mut matches = 0usize;
        while let (Some(&winner), Some(&number)) = (winner_it.peek(), numbers_it.peek()) {
            if winner == number {
                matches += 1;
                numbers_it.next();
                winner_it.next();
            } else if number < winner {
                numbers_it.next();
            } else {
                winner_it.next();
            }
        }

        let my_copies = next_extra_copies.pop_front().unwrap_or(0) + 1;

        total_cards += my_copies;

        next_extra_copies
            .extend(std::iter::repeat(0).take(matches.saturating_sub(next_extra_copies.len())));

        for e in next_extra_copies.iter_mut().take(matches) {
            *e += my_copies;
        }
    }

    println!("day 4 part 2: {}", total_cards);
}
