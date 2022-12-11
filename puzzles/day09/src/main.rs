use std::collections::HashSet;

fn main() {
    let instructions = include_str!("../input.txt")
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
    let mut rope = [(0, 0); 10];
    for (direction, steps) in instructions {
        for _ in 0..*steps {
            let new_head = direction.determine_new_position(rope[0]);
            move_rope(&mut rope, new_head);
            visited_set.insert(rope[9]);
        }
    }
    visited_set.len()
}

fn move_rope(rope: &mut [(i32, i32)], new_head: (i32, i32)) {
    rope[0] = new_head;
    for i in 0..rope.len() - 1 {
        rope[i + 1] = determine_new_position(rope[i], rope[i + 1]);
    }
}

fn determine_new_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return tail;
    }
    let mut new_tail = tail;
    if head.0 == tail.0 {
        new_tail.1 += plus_minus_one(head.1, tail.1);
    } else if head.1 == tail.1 {
        new_tail.0 += plus_minus_one(head.0, tail.0);
    } else {
        new_tail.0 += plus_minus_one(head.0, tail.0);
        new_tail.1 += plus_minus_one(head.1, tail.1);
    }
    new_tail
}

fn plus_minus_one(head: i32, tail: i32) -> i32 {
    if (head - tail).is_positive() {
        1
    } else {
        -1
    }
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