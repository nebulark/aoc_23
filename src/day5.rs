use std::collections::VecDeque;

pub struct Entry {
    dest: i64,
    source: i64,
    len: i64,
}

pub struct Remappings {
    entries: Vec<Entry>,
}

#[derive(Clone, Copy)]
pub struct Range {
    pub pos : i64,
    pub len : i64
}

impl Remappings {
    pub fn map(&self, source: i64) -> i64 {
        let idx = self.entries.partition_point(|e|e.source < source) - 1;

        if let Some(e) = self.entries.get(idx)
        {
            if e.source <= source && source <= e.source + e.len {
                return source - e.source + e.dest;
            }
        }
        
        source
    }

    pub fn map_range(&self, source: Range) -> Vec<Range> {

        let mut curr_range = source;
        let mut res = Vec::new();
        loop {
            if curr_range.len == 0 {
                break res;
            }

            let idx = self.entries.partition_point(|e|e.source < source.len) - 1;
            //        |--------------|
            //      |----|    |--| |-----|
            // |--------------------|

            // if e.source <= curr_range.pos && curr_range.pos <= e.source + e.len {
            //     return source - e.source + e.dest;
            // }



        }



        
    }
}


pub fn part1() {
    let full_file = std::fs::read_to_string("inputs/input_5.txt").unwrap();

    let mut lines = full_file.lines();

    // seeds: 3139431799 50198205 3647185634 110151761 2478641666 139825503 498892555 8913570 961540761 489996751 568452082 100080382 907727477 42158689 1617552130 312026427 342640189 97088268 2049289560 336766062
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();

    //
    lines.next();

    // from now on its always an remap name, mappings, followed by a new line:

    // seed-to-soil map:
    // 1615836342 1401909974 23067952
    // ....
    // 620871316 484454103 121810618

    // soil-to-fertilizer map:
    // 4245401761 2352458099 28057201
    // 2099789767 3998256334 14950546

    let mut mappings = Vec::new();

    loop {
        let Some(name) = lines.next() else {
            break;
        };

        assert!(name.contains("map"));

        let mut entries = Vec::new();
        loop {
            let Some(line) = lines.next() else {
                break;
            };

            if line.trim().is_empty() {
                break;
            }

            let mut vals = line
                .split_ascii_whitespace()
                .map(|e| e.parse::<i64>().unwrap());

            entries.push(Entry {
                dest: vals.next().unwrap(),
                source: vals.next().unwrap(),
                len: vals.next().unwrap(),
            });
        }

        if entries.len() > 0 {

            entries.sort_by_key(|e|e.source);
            mappings.push(Remappings { entries });
        }
    }

    let mut locations: Vec<_> = seeds
        .iter()
        .map(|s| {
            let mut res = *s;
            for m in mappings.iter() {
                res = m.map(res)
            }
            res
        })
        .collect();

    locations.sort();

    println!("day 5 part 1: {}", locations.first().unwrap());
}

pub fn part2() {
    let full_file = std::fs::read_to_string("inputs/input_5.txt").unwrap();

    let mut lines = full_file.lines();

    // seeds: 3139431799 50198205 3647185634 110151761 2478641666 139825503 498892555 8913570 961540761 489996751 568452082 100080382 907727477 42158689 1617552130 312026427 342640189 97088268 2049289560 336766062
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();

    //
    lines.next();

    // from now on its always an remap name, mappings, followed by a new line:

    // seed-to-soil map:
    // 1615836342 1401909974 23067952
    // ....
    // 620871316 484454103 121810618

    // soil-to-fertilizer map:
    // 4245401761 2352458099 28057201
    // 2099789767 3998256334 14950546

    let mut mappings = Vec::new();

    loop {
        let Some(name) = lines.next() else {
            break;
        };

        assert!(name.contains("map"));

        let mut entries = Vec::new();
        loop {
            let Some(line) = lines.next() else {
                break;
            };

            if line.trim().is_empty() {
                break;
            }

            let mut vals = line
                .split_ascii_whitespace()
                .map(|e| e.parse::<i64>().unwrap());

            entries.push(Entry {
                dest: vals.next().unwrap(),
                source: vals.next().unwrap(),
                len: vals.next().unwrap(),
            });
        }

        if entries.len() > 0 {
            entries.sort_by_key(|e|e.source);
            mappings.push(Remappings { entries });
        }
    }

    let mut locations: Vec<_> = seeds
        .iter()
        .map(|s| {
            let mut res = *s;
            for m in mappings.iter() {
                res = m.map(res)
            }
            res
        })
        .collect();

    locations.sort();

    println!("day 5 part 1: {}", locations.first().unwrap());
}
