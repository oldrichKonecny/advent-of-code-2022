use itertools::Itertools;
use std::collections::HashMap;
use std::string::ToString;

const SEPARATOR: &str = "_";

fn main() {
    let input = include_str!("../input.txt");
    let file_system = parse_input(input);

    println!("First part: {}", first_solution(&file_system));
    println!("Second part: {}", second_solution(&file_system));
}

fn second_solution(file_system: &HashMap<String, Option<Directory>>) -> u64 {
    let free_space = 70_000_000 - sum_size_rec(file_system, "/");
    file_system
        .iter()
        .map(|(key, _)| sum_size_rec(file_system, key))
        .filter(|size| *size + free_space >= 30_000_000)
        .sorted()
        .next()
        .unwrap()
}

fn first_solution(file_system: &HashMap<String, Option<Directory>>) -> u64 {
    file_system
        .iter()
        .map(|(key, _)| sum_size_rec(file_system, key))
        .filter(|val| *val <= 100_000)
        .sum()
}

fn sum_size_rec(file_system: &HashMap<String, Option<Directory>>, current_dir: &str) -> u64 {
    let dir = file_system.get(current_dir).unwrap().as_ref().unwrap();
    let mut accumulator = dir.get_size();
    accumulator += dir
        .dirs
        .iter()
        .map(|d| sum_size_rec(file_system, &format!("{}_{}", current_dir, d)))
        .sum::<u64>();
    accumulator
}

fn parse_input(input: &str) -> HashMap<String, Option<Directory>> {
    let mut file_system = HashMap::new();
    let mut path = Vec::new();
    let mut current_short = "/";
    let mut current_full = "/".to_string();
    for line in input.lines() {
        match line {
            l if l.starts_with("$ cd ..") => {
                current_full = path.join(SEPARATOR);
                current_short = path.pop().unwrap();
            }
            l if l.starts_with("$ cd /") => {
                path.drain(..);
                current_short = "/";
                file_system.entry("/".to_string()).or_insert(None);
            }
            l if l.starts_with("$ cd ") => {
                path.push(current_short);
                current_short = l.split(" ").skip(2).next().unwrap();
                current_full = path.join(SEPARATOR) + SEPARATOR + current_short;
                file_system.entry(current_full.clone()).or_insert(None);
            }
            l if l.starts_with("$ ls") => {
                let parent = if path.is_empty() {
                    None
                } else {
                    Some(path.join(SEPARATOR))
                };
                let dir = Directory::new(parent, current_short);
                *file_system.get_mut(&current_full).unwrap() = Some(dir);
            }
            l if l.starts_with("dir ") => {
                let new_dir = l.split(" ").skip(1).next().unwrap();
                file_system
                    .get_mut(&current_full)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .dirs
                    .push(new_dir);
            }
            l => {
                let mut split = l.split(" ");
                let file = File {
                    size: split.next().unwrap().parse().unwrap(),
                    name: split.next().unwrap(),
                };
                file_system
                    .get_mut(&current_full)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .files
                    .push(file);
            }
        }
    }
    file_system
}

#[derive(Debug, Eq, PartialEq)]
struct Directory<'a> {
    parent: Option<String>,
    name: &'a str,
    files: Vec<File<'a>>,
    dirs: Vec<&'a str>,
}

#[derive(Debug, Eq, PartialEq)]
struct File<'a> {
    name: &'a str,
    size: u64,
}

impl<'a> Directory<'a> {
    fn new(parent: Option<String>, name: &'a str) -> Self {
        Directory {
            parent,
            name,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }
}