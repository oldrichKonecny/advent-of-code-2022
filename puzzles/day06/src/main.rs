fn main() {
    let input = include_str!("../input.txt");

    println!(
        "First: part: {}",
        different_bytes_in_input(input, 4).unwrap()
    );
    println!(
        "Second: part: {}",
        different_bytes_in_input(input, 14).unwrap()
    );
}

fn different_bytes_in_input(input: &str, window_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(index, window)| are_elements_different(window))
        .map(|(index, window)| index + window.len())
}

fn are_elements_different(slice: &[u8]) -> bool {
    for i in 0..slice.len() - 1 {
        for j in i + 1..slice.len() {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    true
}