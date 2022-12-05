use once_cell::sync::Lazy;
use regex::Regex;

#[cfg(not(target_os = "windows"))]
const EMPTY_LINE_PATTERN: &str = "\n\n";
#[cfg(target_os = "windows")]
const EMPTY_LINE_PATTERN: &str = "\r\n\r\n";

const SPACE_BYTE_VALUE: u8 = 32;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());

fn main() {
    let (crates, ins) = include_str!("../input.txt")
        .split_once(EMPTY_LINE_PATTERN)
        .unwrap();
    let mut crates = parse_crates(crates);
    let instructions = parse_instruction(ins);

    println!(
        "First part: {}",
        first_solution(&instructions, crates.clone())
    );
    println!("Second part: {}", second_solution(&instructions, crates));
}

fn second_solution(instructions: &[Instruction], mut crates_plan: Vec<Vec<char>>) -> String {
    for instruction in instructions {
        let col = crates_plan.get_mut(instruction.from).unwrap();
        let mut drained = col
            .drain((col.len() - instruction.move_count)..)
            .collect::<Vec<_>>();
        crates_plan
            .get_mut(instruction.to)
            .unwrap()
            .append(&mut drained);
    }

    crates_plan.iter().flat_map(|col| col.last()).collect()
}

fn first_solution(instructions: &[Instruction], mut crates_plan: Vec<Vec<char>>) -> String {
    for instruction in instructions {
        for _ in 0..instruction.move_count {
            let c = crates_plan
                .get_mut(instruction.from)
                .unwrap()
                .pop()
                .unwrap();
            crates_plan.get_mut(instruction.to).unwrap().push(c);
        }
    }

    crates_plan.iter().flat_map(|col| col.last()).collect()
}

fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    let mut map = vec![vec![]];
    for line in crates.lines().rev().skip(1) {
        line.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(index, b)| {
                if map.get(index + 1).is_none() {
                    map.push(Vec::new());
                }
                if b != SPACE_BYTE_VALUE {
                    map.get_mut(index + 1).unwrap().push(b as char);
                }
            });
    }
    map
}

fn parse_instruction(instructions: &str) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|line| {
            let captured = REGEX.captures_iter(line).next().unwrap();
            Instruction {
                move_count: captured[1].parse().unwrap(),
                from: captured[2].parse().unwrap(),
                to: captured[3].parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Instruction {
    move_count: usize,
    from: usize,
    to: usize,
}