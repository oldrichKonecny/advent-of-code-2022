extern crate core;
use std::collections::HashSet;

fn main() {
    let lines = include_str!("../test_input.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();
    let map = compute_line_points(&lines);

    println!(
        "First part: {}",
        simulate_sand_fall(&mut map.clone(), false)
    );
    println!(
        "Second part: {}",
        simulate_sand_fall(&mut map.clone(), true)
    );
}

fn simulate_sand_fall(map: &mut HashSet<(i32, i32)>, bound_floor: bool) -> usize {
    let bound = find_bound(map);
    let bound = bound_floor.then(|| bound + 1).unwrap_or_else(|| bound);
    let mut counter = 0;
    loop {
        match single_sand_fall(map, (500, 0), bound, bound_floor) {
            None => return counter,
            Some(position) => {
                map.insert(position);
                counter += 1;
            }
        }
    }
}

fn single_sand_fall(
    map: &HashSet<(i32, i32)>,
    mut position: (i32, i32),
    bound: i32,
    bound_as_floor: bool,
) -> Option<(i32, i32)> {
    if map.contains(&position) {
        return None;
    }
    loop {
        let expected_position = (position.0, position.1 + 1);
        //check bound
        if bound < expected_position.1 {
            return match bound_as_floor {
                true => Some(position),
                false => None,
            };
        }

        //check underneath
        if map.contains(&expected_position) {
            if !map.contains(&(expected_position.0 - 1, expected_position.1)) {
                position = (expected_position.0 - 1, expected_position.1);
                continue;
            } else if !map.contains(&(expected_position.0 + 1, expected_position.1)) {
                position = (expected_position.0 + 1, expected_position.1);
                continue;
            } else {
                return Some(position);
            }
        }
        position = expected_position;
    }
}

fn find_bound(map: &HashSet<(i32, i32)>) -> i32 {
    let mut bound = 0;
    for (_, y) in map {
        if *y > bound {
            bound = *y;
        }
    }
    bound
}

fn compute_line_points(lines: &Vec<Vec<(i32, i32)>>) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();
    for line in lines {
        for window in line.windows(2) {
            let [a, b] = window else { panic!("dude! what is happening!?") };
            if a.0 == b.0 {
                for i in a.1.min(b.1)..=a.1.max(b.1) {
                    res.insert((a.0, i));
                }
            } else if a.1 == b.1 {
                for i in a.0.min(b.0)..=a.0.max(b.0) {
                    res.insert((i, a.1));
                }
            } else {
                panic!("again! I should not be here!")
            }
        }
    }
    res
}

fn parse_line(line: &str) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(|s| {
            s.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>()
}
