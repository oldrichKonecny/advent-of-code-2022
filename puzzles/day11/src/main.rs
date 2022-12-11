use std::borrow::BorrowMut;

#[cfg(not(target_os = "windows"))]
const EMPTY_LINE_PATTERN: &str = "\n\n";
#[cfg(target_os = "windows")]
const EMPTY_LINE_PATTERN: &str = "\r\n\r\n";

fn main() {
    let mut monkeys = include_str!("../input.txt")
        .split(EMPTY_LINE_PATTERN)
        .map(|raw_monkey| Monkey::parse(raw_monkey))
        .collect::<Vec<_>>();
    let mut first_res = simulate_rounds(&mut monkeys.clone(), 20, |x| x / 3);
    first_res.sort();
    first_res.reverse();
    println!("First part: {}", first_res[0] * first_res[1]);

    let common_divider = monkeys
        .iter()
        .map(|monkey| monkey.divisible_test)
        .reduce(|acc, val| acc * val)
        .unwrap();
    let mut second_res = simulate_rounds(&mut monkeys, 10_000, |x| x % common_divider);
    second_res.sort();
    second_res.reverse();
    println!("Second part: {}", second_res[0] * second_res[1]);
}

fn simulate_rounds<F: Fn(u64) -> u64>(
    monkeys: &mut Vec<Monkey>,
    rounds: u32,
    worry_level_modifier: F,
) -> Vec<usize> {
    let mut monkey_inspect_count = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = monkeys[i].starting_items.drain(..).collect::<Vec<_>>();
            monkey_inspect_count[i] += items.len();
            let throw_if_true = monkeys[i].throw_if_true;
            let throw_if_false = monkeys[i].throw_if_false;

            for worry_level in items.iter().rev() {
                let mut worry_level = monkeys[i].operation.compute(*worry_level);
                // worry_level = worry_level % common_divider;
                worry_level = worry_level_modifier(worry_level);
                if worry_level % monkeys[i].divisible_test == 0 {
                    monkeys[throw_if_true].starting_items.insert(0, worry_level);
                } else {
                    monkeys[throw_if_false]
                        .starting_items
                        .insert(0, worry_level);
                }
            }
        }
    }
    monkey_inspect_count
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines().skip(1);
        let starting_items = lines
            .next()
            .unwrap()
            .split_once("Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let operation = Operation::parse(
            lines
                .next()
                .unwrap()
                .split_once("Operation: new = ")
                .unwrap()
                .1,
        );
        let divisible_test = lines
            .next()
            .unwrap()
            .split_once("Test: divisible by ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let throw_if_true = lines
            .next()
            .unwrap()
            .split_once("If true: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let throw_if_false = lines
            .next()
            .unwrap()
            .split_once("If false: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        Self {
            starting_items,
            operation,
            divisible_test,
            throw_if_true,
            throw_if_false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Addition(u64),
    Multiplication(u64),
    Square,
}

impl Operation {
    fn parse(line: &str) -> Self {
        match line {
            l if l.starts_with("old * old") => Operation::Square,
            l if l.starts_with("old * ") => {
                Operation::Multiplication(l.split_once("old * ").unwrap().1.parse().unwrap())
            }
            l if l.starts_with("old + ") => {
                Operation::Addition(l.split_once("old + ").unwrap().1.parse().unwrap())
            }
            l => panic!("Cannot parse operation [{}]", l),
        }
    }

    fn compute(&self, n: u64) -> u64 {
        match self {
            Operation::Addition(x) => n + *x,
            Operation::Multiplication(x) => n * *x,
            Operation::Square => n * n,
        }
    }
}