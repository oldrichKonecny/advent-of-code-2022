use std::str::from_utf8;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");

    println!("First part: {}", first_solution(input));
    println!("Second part: {}", second_solution(input));

    let end = start.elapsed();
    println!("Duration:");
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());
}

fn second_solution(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| {
            let mut iter = chunk.iter();
            let first = iter.next().unwrap();
            let sec = iter.next().unwrap();
            let third = iter.next().unwrap();
            *first
                .iter()
                .filter(|i| sec.contains(i))
                .collect::<Vec<_>>()
                .iter()
                .filter(|i| third.contains(i))
                .next()
                .expect("Didnt found any common letter")
        })
        .map(correct_value)
        .sum()
}

fn first_solution(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.as_bytes().split_at(l.len() / 2))
        .map(|(left, right)| {
            for i in left {
                for j in right {
                    if *i == *j {
                        return i;
                    }
                }
            }
            panic!(
                "Didnt find match in left: {:?}, right: {:?}",
                from_utf8(left),
                from_utf8(right)
            );
        })
        .map(correct_value)
        .sum::<u64>()
}

fn correct_value(val: &u8) -> u64 {
    let res = if (*val as char).is_uppercase() {
        *val - 38
    } else {
        *val - 96
    };
    res as u64
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        assert_eq!(2, 1 + 1);
    }
}
