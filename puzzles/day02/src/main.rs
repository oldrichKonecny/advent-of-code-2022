use std::time::Instant;

fn main() {
    let mut start = Instant::now();
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
    input.lines()
        .map(|l| {
            let mut split = l.split(" ");
            let mut opponent = split.next().expect("Cannot get first letter");
            let mut outcome = split.next().expect("Cannot get second letter");
            (Outcome::parse(outcome), Choices::parse(opponent))
        })
        .map(|(outcome, opponent_choice)| {
            let res = match outcome {
                Outcome::Win => 6 + opponent_choice.lose_to().value(),
                Outcome::Lose => opponent_choice.win_to().value(),
                Outcome::Draw => 3 + opponent_choice.value(),
            };
            res as u64
        })
        .sum()
}

fn first_solution(input: &str) -> u64 {
    input.lines()
        .map(|l| {
            let mut split = l.split(" ");
            let opponent = split.next().expect("Cannot get first letter");
            let should_play = split.next().expect("Cannot get second letter");
            (Choices::parse(should_play), Choices::parse(opponent))
        })
        .map(|(my_choice, opponent_choice)| compute_outcome(&my_choice, &opponent_choice) as u64)
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn parse(letter: &str) -> Self {
        match letter {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Cannot parse letter '{}' to Outcome", letter)
        }
    }

    fn value(&self) -> u8 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl Choices {
    fn parse(letter: &str) -> Self {
        match letter {
            "A" | "X" => Choices::Rock,
            "B" | "Y" => Choices::Paper,
            "C" | "Z" => Choices::Scissors,
            _ => panic!("Cannot parse letter '{}' to Choices", letter)
        }
    }

    fn lose_to(&self) -> Self {
        match self {
            Choices::Rock => Choices::Paper,
            Choices::Paper => Choices::Scissors,
            Choices::Scissors => Choices::Rock,
        }
    }

    fn win_to(&self) -> Self {
        match self {
            Choices::Rock => Choices::Scissors,
            Choices::Paper => Choices::Rock,
            Choices::Scissors => Choices::Paper,
        }
    }

    fn value(&self) -> u8 {
        match self {
            Choices::Rock => 1,
            Choices::Paper => 2,
            Choices::Scissors => 3,
        }
    }
}

fn compute_outcome(my_choice: &Choices, opponent_choice: &Choices) -> u8 {
    if *my_choice == *opponent_choice {
        return my_choice.value() + 3;
    }
    match (my_choice, opponent_choice) {
        (Choices::Rock, Choices::Scissors) | (Choices::Paper, Choices::Rock) | (Choices::Scissors, Choices::Paper) => my_choice.value() + 6,
        _ => my_choice.value()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2, 1 + 1);
    }
}
