#[cfg(not(target_os = "windows"))]
const EMPTY_LINE_PATTERN: &str = "\n\n";
#[cfg(target_os = "windows")]
const EMPTY_LINE_PATTERN: &str = "\r\n\r\n";

const SPACE_BYTE_VALUE: u8 = 32;

fn main() {
    let (crates, instruction) = include_str!("../input.txt")
        .split_once(EMPTY_LINE_PATTERN)
        .unwrap();
    let mut crates = parse_crates(crates);
    println!("{:?}", crates);
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
    todo!()
}

struct Instruction {
    move_count: u32,
    from: usize,
    to: usize,
}