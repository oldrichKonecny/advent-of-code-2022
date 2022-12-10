fn main() {
    let instructions = include_str!("../input.txt")
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();
    let timeline = compute_timeline(&instructions);

    println!("First part: {}", first_solution(&timeline));
    println!("Second part:");
    println!("{}", second_solution(&timeline));
}

fn second_solution(timeline: &[i32]) -> String {
    let mut res = String::with_capacity(250);
    let mut sprite;

    for i in 0..240 {
        let val = i % 40;
        let next_sprite_start = timeline[i as usize];
        sprite = next_sprite_start..next_sprite_start + 3;
        if sprite.contains(&(val + 1)) {
            res.push('#');
        } else {
            res.push('.');
        }
        if (i + 1) % 40 == 0 {
            res.push('\n');
        }
    }

    res
}

fn first_solution(timeline: &[i32]) -> i32 {
    let important_cycles = [20i32, 60, 100, 140, 180, 220];
    important_cycles
        .iter()
        .map(|&i| i * timeline[(i - 1) as usize])
        .sum::<i32>()
}

fn compute_timeline(instructions: &[Instruction]) -> [i32; 240] {
    let mut res = [0; 240];
    let mut index = 0;
    let mut x = 1;
    for ins in instructions {
        match ins {
            Instruction::Noop => {
                res[index] = x;
                index += 1;
            }
            Instruction::Addx(n) => {
                res[index] = x;
                res[index + 1] = x;
                index += 2;
                x += *n;
            }
        }
    }

    res
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        match line {
            "noop" => Instruction::Noop,
            line => Instruction::Addx(line.split_once(" ").unwrap().1.parse().unwrap()),
        }
    }
}