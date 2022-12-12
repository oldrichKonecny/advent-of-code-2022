use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut map = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().map(Node::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_coordinates = find_start(&map).unwrap();

    let mut first_map = map.clone();
    do_bfs(&mut first_map, start_coordinates);
    let end_node = find_end_path_cost(&first_map);
    println!("First part: {}", end_node.unwrap().unwrap());

    let all_a = find_all_a(&map);
    let mut minimum = u32::MAX;
    for start in all_a {
        let mut new_map = map.clone();
        do_bfs(&mut new_map, start);
        let cost = find_end_path_cost(&new_map);
        if cost.is_ok() && cost.unwrap().is_some() && cost.unwrap().unwrap() < minimum {
            minimum = cost.unwrap().unwrap();
        }
    }
    println!("Second part: {}", minimum);

    let end = start.elapsed();
    println!("Duration:");
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());
}

fn find_all_a(map: &Vec<Vec<Node>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (index_i, col) in map.iter().enumerate() {
        for (index_j, node) in col.iter().enumerate() {
            if node.height == 1 {
                res.push((index_i, index_j))
            }
        }
    }
    res
}

fn do_bfs(map: &mut Vec<Vec<Node>>, start: (usize, usize)) {
    let row_length = map[0].len();
    let col_length = map.len();
    let mut queue = VecDeque::new();
    queue.push_front(start);
    map[start.0][start.1].path_cost = Some(0);

    while !queue.is_empty() {
        let (x, y) = queue.pop_back().unwrap();
        let current_height = map[x][y].height;
        let current_cost = map[x][y].path_cost.unwrap();

        map[x][y].visited = true;

        if x > 0 {
            determine_cost(map, (x - 1, y), current_cost, current_height, &mut queue);
        }
        if x < col_length - 1 {
            determine_cost(map, (x + 1, y), current_cost, current_height, &mut queue);
        }
        if y > 0 {
            determine_cost(map, (x, y - 1), current_cost, current_height, &mut queue);
        }
        if y < row_length - 1 {
            determine_cost(map, (x, y + 1), current_cost, current_height, &mut queue);
        }
    }
}

fn determine_cost(
    map: &mut Vec<Vec<Node>>,
    target: (usize, usize),
    current_cost: u32,
    current_height: u8,
    queue: &mut VecDeque<(usize, usize)>,
) {
    let node = &mut map[target.0][target.1];
    if node.height > current_height + 1 {
        return;
    }
    if node.path_cost.is_none() || node.path_cost.unwrap() > current_cost + 1 {
        node.path_cost = Some(current_cost + 1);
    }
    if !node.visited && !queue.contains(&(target.0, target.1)) {
        queue.push_front((target.0, target.1));
    }
}

fn find_end_path_cost(map: &Vec<Vec<Node>>) -> Result<Option<u32>, ()> {
    for col in map.iter() {
        for node in col.iter() {
            if node.marker == Marker::End {
                return Ok(node.path_cost);
            }
        }
    }
    Err(())
}

fn find_start(map: &Vec<Vec<Node>>) -> Result<(usize, usize), ()> {
    for (index_i, col) in map.iter().enumerate() {
        for (index_j, node) in col.iter().enumerate() {
            if node.marker == Marker::Start {
                return Ok((index_i, index_j));
            }
        }
    }
    Err(())
}

#[derive(Debug, Clone)]
struct Node {
    path_cost: Option<u32>,
    visited: bool,
    height: u8,
    marker: Marker,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Marker {
    Start,
    End,
    Normal,
}

impl Node {
    fn from(c: char) -> Self {
        let mut marker = Marker::Normal;
        let height = match c {
            'S' => {
                marker = Marker::Start;
                1
            }
            'E' => {
                marker = Marker::End;
                26
            }
            c => c as u8 - 96,
        };
        Self {
            path_cost: None,
            visited: false,
            height,
            marker,
        }
    }
}
