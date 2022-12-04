use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    println!("First part: {}", first_solution(&input));
    println!("Second part: {}", second_solution(&input));

    let end = start.elapsed();
    println!("Duration:");
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());
}

fn first_solution(pairs: &[((u32, u32), (u32, u32))]) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| {
            (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1)
        })
        .count()
}

fn second_solution(pairs: &[((u32, u32), (u32, u32))]) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| {
            let left = left.0..=left.1;
            let right = right.0..=right.1;
            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count()
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    fn parse_pair(pair: &str) -> (u32, u32) {
        let mut nums = pair.split("-");
        let first = nums.next().unwrap().parse().unwrap();
        let sec = nums.next().unwrap().parse().unwrap();
        (first, sec)
    }
    let mut pairs = line.split(",");
    let first = parse_pair(pairs.next().unwrap());
    let sec = parse_pair(pairs.next().unwrap());
    (first, sec)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        assert_eq!(2, 1 + 1);
    }
}