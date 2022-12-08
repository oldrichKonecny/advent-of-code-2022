fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible_trees = 0u64;
    let mut scenic_score = 0u64;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if tree_is_visible(&grid, (i, j)) {
                visible_trees += 1;
            }
            let new_score = compute_scenic_score(&grid, (i, j));
            if scenic_score < new_score {
                scenic_score = new_score;
            }
        }
    }

    println!("First part: {}", visible_trees);
    println!("Second part: {}", scenic_score);
}

fn compute_scenic_score(grid: &[Vec<u8>], coordinates: (usize, usize)) -> u64 {
    let (x, y) = coordinates;
    let row = grid.get(x).unwrap();
    if x == 0 || x == row.len() - 1 || y == 0 || y == grid.len() - 1 {
        return 0;
    }
    let col = grid
        .iter()
        .map(|vec| *vec.get(y).unwrap())
        .collect::<Vec<_>>();

    let target = *row.get(y).unwrap();

    find_scenic_score(target, &row[0..y], true)
        * find_scenic_score(target, &row[y + 1..], false)
        * find_scenic_score(target, &col[0..x], true)
        * find_scenic_score(target, &col[x + 1..], false)
}

fn find_scenic_score(tree_height: u8, vec: &[u8], is_reverse: bool) -> u64 {
    let mut score = 0;
    if is_reverse {
        for val in vec.iter().rev() {
            score += 1;
            if tree_height <= *val {
                break;
            }
        }
    } else {
        for val in vec.iter() {
            score += 1;
            if tree_height <= *val {
                break;
            }
        }
    }

    score
}

fn tree_is_visible(grid: &[Vec<u8>], coordinates: (usize, usize)) -> bool {
    let (x, y) = coordinates;
    let row = grid.get(x).unwrap();

    if x == 0 || x == row.len() - 1 || y == 0 || y == grid.len() - 1 {
        return true;
    }

    let target = *row.get(y).unwrap();

    if !bigger_or_same_tree_exist(target, &row[0..y])
        || !bigger_or_same_tree_exist(target, &row[y + 1..])
    {
        return true;
    }

    let col = grid
        .iter()
        .map(|vec| *vec.get(y).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(target, *col.get(x).unwrap());
    if !bigger_or_same_tree_exist(target, &col[0..x])
        || !bigger_or_same_tree_exist(target, &col[x + 1..])
    {
        return true;
    }

    false
}

fn bigger_or_same_tree_exist(tree_height: u8, other_trees: &[u8]) -> bool {
    other_trees.iter().any(|u| *u >= tree_height)
}

#[cfg(test)]
mod tests {
    use crate::bigger_or_same_tree_exist;

    #[test]
    fn test_bigger_or_same_tree_exist() {
        assert!(bigger_or_same_tree_exist(5, &[5]));
        assert!(bigger_or_same_tree_exist(5, &[7]));
        assert!(bigger_or_same_tree_exist(5, &[1, 2, 0, 5, 5]));
        assert!(bigger_or_same_tree_exist(5, &[1, 2, 9, 0, 5, 5]));
        assert!(bigger_or_same_tree_exist(0, &[0]));

        assert!(!bigger_or_same_tree_exist(5, &[]));
        assert!(!bigger_or_same_tree_exist(9, &[1, 0, 5, 8]));
        assert!(!bigger_or_same_tree_exist(0, &[]));
        assert!(!bigger_or_same_tree_exist(1, &[0, 0, 0]));
        assert!(!bigger_or_same_tree_exist(
            5,
            &[4, 1, 0, 2, 3, 4, 4, 4, 4, 4]
        ));
    }
}