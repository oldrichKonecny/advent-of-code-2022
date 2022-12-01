use std::collections::BinaryHeap;

fn main() {
    let mut heap = include_str!("../input.txt")
        .split("\n\n")
        .map(parse_and_sum_numbers)
        .collect::<BinaryHeap<u64>>();

    println!("First part: {}", heap.peek().unwrap());

    let top3 = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();

    println!("Second part: {}", top3);
}

fn parse_and_sum_numbers(lines: &str) -> u64 {
    lines.lines()
        .map(|l| l.parse::<u64>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        assert_eq!(2, 1 + 1);
    }

}
