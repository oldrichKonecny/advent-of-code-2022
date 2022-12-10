use std::collections::{HashMap, HashSet};

fn main() {
    let instructions = include_str!("../test_input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(direction, steps)| (Direction::parse(direction), steps.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    println!("First part: {}", first_solution(&instructions));
    println!("Second part: {}", second_solution(&instructions));
}

fn second_solution(instructions: &[(Direction, i32)]) -> usize {
    let mut visited_set = HashSet::new();
    visited_set.insert((0, 0));
    let mut rope_position = [(0, 0); 10];
    for (direction, steps) in instructions {
        println!("{:?} -> {}", direction, steps);
        for _ in 0..*steps {
            let mut old_head = rope_position[0];
            rope_position[0] = direction.determine_new_position(old_head);
            for i in 0..rope_position.len() - 1 {
                let new_head = rope_position[i];
                let tail = rope_position[i + 1];
                // println!(
                //     "new_head: {:?}, old_head: {:?}, tail: {:?}",
                //     new_head, old_head, tail
                // );
                // println!(
                //     "should move tail {:?} -> new_head {:?} = {}",
                //     tail,
                //     new_head,
                //     tail_should_move(tail, new_head)
                // );
                if tail_should_move(tail, new_head) {
                    rope_position[i + 1] = old_head;
                }
                old_head = tail;
            }
            println!("{:?}", rope_position);
            *visited_set.entry(rope_position[9]).or_insert(1) += 1;
        }
        println!();
        println!();
    }
    visited_set.len()
}

fn first_solution(instructions: &[(Direction, i32)]) -> usize {
    let mut visited_set = HashSet::new();
    visited_set.insert((0, 0));
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    for (direction, steps) in instructions {
        for _ in 0..*steps {
            let new_head_position = direction.determine_new_position(head_position);
            if tail_should_move(tail_position, new_head_position) {
                tail_position = head_position;
                visited_set.insert(tail_position);
            }
            head_position = new_head_position;
        }
    }
    visited_set.len()
}

fn tail_should_move(tail_pos: (i32, i32), new_head_pos: (i32, i32)) -> bool {
    (tail_pos.0 - new_head_pos.0).abs() > 1 || (tail_pos.1 - new_head_pos.1).abs() > 1
}

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn parse(input: &str) -> Self {
        match input {
            "L" => Self::L,
            "R" => Self::R,
            "U" => Self::U,
            "D" => Self::D,
            i => panic!("Unexpected input {}, cannot parse to Direction", i),
        }
    }

    fn determine_new_position(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::L => (pos.0 - 1, pos.1),
            Direction::R => (pos.0 + 1, pos.1),
            Direction::U => (pos.0, pos.1 + 1),
            Direction::D => (pos.0, pos.1 - 1),
        }
    }
}