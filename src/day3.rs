use std::ops::Add;

#[derive(Debug)]
struct Entry {
    row: usize,
    cstart: usize,
    clast: usize,
    val: u32,
}

pub fn part1() {
    // 467..114..
    // ...*......
    // ..35..633.

    // number - value, line, colum-colum

    let mut symbols = Vec::new();
    let mut entries = Vec::new();

    for (row, line) in std::fs::read_to_string("inputs/input_3.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let mut symbol_line = Vec::new();

        let mut current_number: Option<Entry> = None;

        for (column, c) in line.char_indices() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(cn) = &mut current_number {
                    cn.val = cn.val * 10 + digit;
                    cn.clast = column;
                } else {
                    current_number = Some(Entry {
                        row: row,
                        cstart: column,
                        clast: column,
                        val: digit,
                    });
                }

                symbol_line.push(false);
            } else {
                if let Some(cn) = current_number {
                    entries.push(cn);
                    current_number = None;
                }

                symbol_line.push(c != '.');
            }
        }

        if let Some(cn) = current_number {
            entries.push(cn);
        }
        symbols.push(symbol_line);
    }

    let mut sum = 0;

    for e in entries {
        let mut is_valid = false;

        //let do_print = e.row == 139 && e.val == 795;

        for row in symbols[e.row.saturating_sub(1)..(e.row + 2).min(symbols.len())].iter() {
            let colums = &row[e.cstart.saturating_sub(1)..(e.clast + 2).min(row.len())];
            // if do_print {
            //     println!("Row: [{}..{}] {:?}", e.cstart.saturating_sub(1), e.clast+1, colums);
            // }

            is_valid = colums.iter().any(|b| *b);
            if is_valid {
                break;
            }
        }

        if is_valid {
            // println!("valid entry {:?}", e);
            sum += e.val;
        } else {
            // println!("invalid entry {:?}", e);

            // for row in symbols[e.row.saturating_sub(1)..(e.row+2).min(symbols.len())].iter()
            // {
            //     let colums = &row[e.cstart.saturating_sub(1)..(e.clast + 2).min(row.len())];

            //     println!("{:?}", colums);

            // }
        }
    }

    println!("day 3 part 1: {}", sum);
}

pub fn part2() {
    let mut numbers = Vec::new();
    let mut number_index_grid = Vec::new();

    let mut gears = Vec::new();

    for (row, line) in std::fs::read_to_string("inputs/input_3.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let mut number_index_row = Vec::new();
        let mut current_number: Option<u32> = None;

        for (column, c) in line.char_indices() {
            if let Some(digit) = c.to_digit(10) {
                number_index_row.push(numbers.len());

                if let Some(cn) = &mut current_number {
                    *cn = *cn * 10 + digit;
                } else {
                    current_number = Some(digit);
                }
            } else {
                number_index_row.push(usize::MAX);
                if let Some(cn) = current_number {
                    numbers.push(cn);
                    current_number = None;
                }

                if c == '*' {
                    gears.push((row, column));
                }
            }
        }

        if let Some(cn) = current_number {
            numbers.push(cn);
        }
        number_index_grid.push(number_index_row);
    }

    let mut sum = 0;

    let mut adjacent_indices = Vec::new();
    for (gear_row, gear_colum) in gears {
        adjacent_indices.clear();

        for row in number_index_grid
            [gear_row.saturating_sub(1)..gear_row.add(2).min(number_index_grid.len())]
            .iter()
        {
            let slice = &row[gear_colum.saturating_sub(1)..gear_colum.add(2).min(row.len())];
            adjacent_indices.extend_from_slice(slice);
        }

        adjacent_indices.retain(|e| *e != usize::MAX);
        adjacent_indices.sort();
        adjacent_indices.dedup();

        if adjacent_indices.len() == 2 {
            let a = numbers[adjacent_indices[0]];
            let b = numbers[adjacent_indices[1]];

            sum += a * b;

            // println!("gear({},{}) = {} x {} = {}", gear_row, gear_colum, a, b, a*b);
        } else {

            // let nums : Vec<_> = adjacent_indices.iter().map(|i|numbers[*i]).collect();
            // println!("invalid gear ({},{}) = {:?} ", gear_row, gear_colum, nums);
        }
    }

    println!("day 3 part 2: {}", sum);
}
