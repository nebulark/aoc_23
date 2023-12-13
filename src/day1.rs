pub fn part1() {
    let mut sum = 0;
    for line in std::fs::read_to_string("inputs/input_1.txt")
        .unwrap()
        .lines()
    {
        sum += line
            .chars()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap()
            * 10;
        sum += line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
    }

    println!("day 1 part 1: {}", sum);
}

pub fn part2() {
    let lut = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let mut sum = 0;

    for line in std::fs::read_to_string("inputs/input_1.txt")
        .unwrap()
        .lines()
    {
        let first = {
            let mut val = 0;
            let mut idx = usize::MAX;
            for (lut_idx, s) in lut.iter().enumerate() {
                if let Some((new_idx, _)) = line.match_indices(s).next() {
                    if new_idx < idx {
                        idx = new_idx;
                        val = lut_idx / 2 + 1;
                    }
                }
            }
            val
        };

        let last = {
            let mut val = 0;
            let mut idx = 0;
            for (lut_idx, s) in lut.iter().enumerate() {
                if let Some((new_idx, _)) = line.match_indices(s).last() {
                    if new_idx >= idx {
                        idx = new_idx;
                        val = lut_idx / 2 + 1;
                    }
                }
            }

            val
        };

        sum += first * 10 + last;
    }
    println!("day 1 part 2: {}", sum);
}
