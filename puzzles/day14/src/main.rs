fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();
}

fn parse_line(line: &str) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(|s| {
            s.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>()
}
