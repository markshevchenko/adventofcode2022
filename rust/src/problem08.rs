fn is_visible(trees: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let tree_height = trees[row][column];
    let mut left_visible = true;
    let mut top_visible = true;
    let mut right_visible = true;
    let mut bottom_visible = true;
    for i in 0..column {
        if trees[row][i] >= tree_height {
            left_visible = false;
            break;
        } 
    }
    for i in (column + 1)..trees[row].len() {
        if trees[row][i] >= tree_height {
            right_visible = false;
            break;
        }
    }
    for i in 0..row {
        if trees[i][column] >= tree_height {
            top_visible = false;
            break;
        }
    }
    for i in (row + 1)..trees.len() {
        if trees[i][column] >= tree_height {
            bottom_visible = false;
            break;
        }
    }
    
    left_visible || top_visible || right_visible || bottom_visible
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let mut trees = Vec::new();
    for line in lines {
        let bytes = line.bytes().collect::<Vec<_>>();
        trees.push(bytes);
    }
    
    let width = trees[0].len();
    let height = trees.len();
    
    let mut count = 2 * (height + width) - 4;
    
    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            if is_visible(&trees, i, j) {
                count += 1
            }
        }
    }
    
    count
}

#[test]
fn solve_a_with_sample_data_returns_21() {
    let sample = indoc::indoc!("
        30373
        25512
        65332
        33549
        35390");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(21, actual);
}

fn get_scenic_score(trees: &Vec<Vec<u8>>, row: usize, column: usize) -> usize {
    let tree_height = trees[row][column];
    let mut left_score = 0;
    let mut top_score = 0;
    let mut right_score = 0;
    let mut bottom_score = 0;
    
    for i in (0..column).rev() {
        left_score += 1;
        if trees[row][i] >= tree_height {
            break;
        }
    }
    for i in (column + 1)..trees[row].len() {
        right_score += 1;
        if trees[row][i] >= tree_height {
            break;
        }
    }
    for i in (0..row).rev() {
        top_score += 1;
        if trees[i][column] >= tree_height {
            break;
        }
    }
    for i in (row + 1)..trees.len() {
        bottom_score += 1;
        if trees[i][column] >= tree_height {
            break;
        }
    }
    
    left_score * top_score * right_score * bottom_score
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let mut trees = Vec::new();
    for line in lines {
        let bytes = line.bytes().collect::<Vec<_>>();
        trees.push(bytes);
    }

    let width = trees[0].len();
    let height = trees.len();

    let mut max_scenic_score = 0;

    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            let scenic_score = get_scenic_score(&trees, i, j);
            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score;
            } 
        }
    }

    max_scenic_score
}

#[test]
fn solve_b_with_sample_data_returns_8() {
    let sample = indoc::indoc!("
        30373
        25512
        65332
        33549
        35390");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(8, actual);
}
