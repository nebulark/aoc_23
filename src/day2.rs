pub fn part1() {
    let colors_max = [12, 13, 14];

    let mut sum = 0;
    'line: for (line_idx, line) in std::fs::read_to_string("inputs/input_2.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        // Game 1: 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue
        let data = line.split_once(':').unwrap().1.trim();
        // 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue
        for set in data.split(';') {
            // 4 red, 3 blue
            for cubes in set.split(',') {
                //3 blue
                let s = cubes.trim().split_once(' ').unwrap();
                let num: u32 = s.0.parse().unwrap();
                let color = match s.1 {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    x => panic!("{} | {} | line {}", x, data, line_idx + 1),
                };

                if num > colors_max[color] {
                    continue 'line;
                }
            }
        }
        sum += line_idx + 1;
    }

    println!("day 2 part 1: {}", sum);
}

pub fn part2() {
    let mut sum = 0;
    for (line_idx, line) in std::fs::read_to_string("inputs/input_2.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        // Game 1: 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue
        let data = line.split_once(':').unwrap().1.trim();
        // 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue
        let mut colors_min = [0, 0, 0];
        for set in data.split(';') {
            // 4 red, 3 blue
            for cubes in set.split(',') {
                //3 blue
                let s = cubes.trim().split_once(' ').unwrap();
                let num: u32 = s.0.parse().unwrap();
                let color = match s.1 {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    x => panic!("{} | {} | line {}", x, data, line_idx + 1),
                };

                colors_min[color] = colors_min[color].max(num);
            }
        }
        sum += colors_min[0] * colors_min[1] * colors_min[2];
    }

    println!("day 2 part 2: {}", sum);
}
