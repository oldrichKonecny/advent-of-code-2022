fn main() {
    let map = include_str!("../test_input.txt")
        .lines()
        .map(|line| line.);
}

struct Node {
    path_cost: u32,
    visited: bool,
    height: u8,

}