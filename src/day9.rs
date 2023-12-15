use itertools::Itertools;


pub fn part1() {
    fn solve(input : &[i32]) -> i32
    {
        let mut lines = vec![input.to_vec()];

        loop {
            let next_line : Vec<i32> = lines.last().unwrap().windows(2).map(|w| {
                let [a, b] = w else { panic!();};
                b - a
            }).collect();

            if next_line.iter().all(|e|*e== 0) {
                break;
            }

            lines.push(next_line);
        }

       lines.iter().map(|v|v.last().unwrap()).sum()
    }  

    let mut sum = 0;

    for (line_idx, line) in std::fs::read_to_string("inputs/input_9.txt")
        .unwrap()
        .lines()
        .enumerate()
    { 
        // 12 23 42 82 168 344 680 1276 2260 3777 5966
        let linevec : Vec<_> = line.split_ascii_whitespace().map(|s|s.parse().unwrap()).collect();
        sum += solve(&linevec);
    }

    println!("day 9 part 1: {}", sum);
}

pub fn part2() {
    fn solve(input : &[i32]) -> i32
    {
        let mut lines = vec![input.to_vec()];

        loop {
            let next_line : Vec<i32> = lines.last().unwrap().windows(2).map(|w| {
                let [a, b] = w else { panic!();};
                b - a
            }).collect();

            if next_line.iter().all(|e|*e== 0) {
                break;
            }

            lines.push(next_line);
        }

        // 2 - 0 = 2
        // 0 - (2 - 0) = - 2
        // 3 - (0 - (2 - 0)) = 5
        // 10 - (3 - (0 - (2 - 0))) = 5

        lines.iter().rev().map(|v|v.first().unwrap()).fold(0, |acc,x| x - acc)      
    }  

    let mut sum = 0;

    for (line_idx, line) in std::fs::read_to_string("inputs/input_9.txt")
        .unwrap()
        .lines()
        .enumerate()
    { 
        // 12 23 42 82 168 344 680 1276 2260 3777 5966
        let linevec : Vec<_> = line.split_ascii_whitespace().map(|s|s.parse().unwrap()).collect();
        sum += solve(&linevec);
    }

    println!("day 9 part 2: {}", sum);
}
