use std::collections::HashMap;



pub fn part1() {
    let full_file = std::fs::read_to_string("inputs/input_8.txt").unwrap();

    let mut lines = full_file.lines();

    // LRLRLLRLLRRRLRLLRRLRLRR
    let go_right_instructions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect();

    //
    lines.next();

    fn tok_to_index(s : &str) -> u32 {
        // 5 bit per char as we never go over 32 (2^5)
        s.chars().map(|c|match c
            {
                x @ 'A'..='Z' => (x as u32) - ('A' as u32),
                y => panic!("{}", y),
            }           
        ).fold(0, |acc, x|(acc << 5) | x)
    }

    let start_idx = tok_to_index("AAA");
    let finish_idx = tok_to_index("ZZZ");

    let mut network = HashMap::new();

    for line in lines {
        // LNR = (DQG, CMF)

        let index = tok_to_index(&line[0..=2]);
        let left = tok_to_index(&line[7..=9]);
        let right = tok_to_index(&line[12..=14]);

        network.insert(index, [left, right]);      
    }

    let mut current = start_idx;

    let mut steps = 0;

    'outer: loop {
        for &go_right in go_right_instructions.iter(){
            if current == finish_idx {
                break 'outer;
            }

            current = network.get(&current).unwrap()[go_right];
            steps += 1;
        }
    }   

    println!("day 8 part 1: {}", steps);
}

pub fn part2() {
    let full_file = std::fs::read_to_string("inputs/input_8.txt").unwrap();

    let mut lines = full_file.lines();

    // LRLRLLRLLRRRLRLLRRLRLRR
    let go_right_instructions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect();

    //
    lines.next();

    fn tok_to_index(s : &str) -> u32 {
        // 5 bit per char as we never go over 32 (2^5)
        s.chars().map(|c|match c
            {
                x @ 'A'..='Z' => (x as u32) - ('A' as u32),
                y => panic!("{}", y),
            }           
        ).fold(0, |acc, x|(acc << 5) | x)
    }

    let mut start_indices = Vec::new();
    let mut end_indices = Vec::new();

    let mut network = HashMap::new();
    let mut idx_to_str = HashMap::new();

    for line in lines {
        // LNR = (DQG, CMF)

        // direct idexing works because we are just in ascii range and don't have to consider UTF8 stuff

        let index = tok_to_index(&line[0..=2]);
        match line.chars().nth(2).unwrap() {
            'A' => { start_indices.push(index); },
            'Z' => { end_indices.push(index); },
            _ => (),       
        }

        let left = tok_to_index(&line[7..=9]);
        let right = tok_to_index(&line[12..=14]);
        idx_to_str.insert(index, line[0..=2].to_string());
        network.insert(index, [left, right]);      
    }

    let mut current = start_indices;
    let mut current_str : Vec<_> = current.iter().map(|i|idx_to_str.get(i).unwrap()).collect();

    println!("Indices {}", current.len());
    println!("end indices {:?}", end_indices);

    let mut steps = 0;

    'outer: loop {
        for &go_right in go_right_instructions.iter(){

            let num_matches : i32 = current.iter().map(|c|end_indices.contains(c) as i32).sum();
            current_str.clear();
            current_str.extend(current.iter().map(|i|idx_to_str.get(i).unwrap()));
            println!("current {:?} \t matchces {}", current_str, num_matches);

            if current.iter().all(|c|end_indices.contains(c)) {
                break 'outer;
            }

            for c in current.iter_mut() {
                *c = network.get(&c).unwrap()[go_right];
            }

            steps += 1;

            // if steps > 20 { break 'outer;}
        }
    }   

    println!("day 8 part 2: {}", steps);
}
