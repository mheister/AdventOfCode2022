use std::{env, fs};

use itertools::Itertools;

struct SideVisibility(Vec<bool>);

impl SideVisibility {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn update_from_line(&mut self, ln: &str) {
        self.0.resize(ln.len(), false);
        self.0.fill(false);
        self.mark_visibles_from_left(&ln);
        self.mark_visibles_from_right(&ln);
    }
    fn mark_visibles_from_left(&mut self, ln: &str) {
        let mut tallest = -1;
        for (height, flag) in ln
            .chars()
            .map(|ch| ch.to_digit(10).expect("Got a non-digit?") as i32)
            .zip(self.0.iter_mut())
        {
            if height > tallest {
                *flag = true;
                tallest = height;
            }
        }
    }
    // FIXME: Deduplicate somehow
    fn mark_visibles_from_right(&mut self, ln: &str) {
        let mut tallest = -1;
        for (height, flag) in ln
            .chars()
            .rev()
            .map(|ch| ch.to_digit(10).expect("Got a non-digit?") as i32)
            .zip(self.0.iter_mut().rev())
        {
            if height > tallest {
                *flag = true;
                tallest = height;
            }
        }
    }
    fn count(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }
}

#[derive(Debug, Clone)]
struct TreeColumnAccumulatorP1 {
    // left_view_blocked: bool,
    tallest: i32,

    // number of trees visible from top and/or bottom but not from the sides
    visible_from_top: usize,

    // flags indicating that there is a tree of the given height visible exclusively
    // from below
    visible_from_below_only: [bool; 10],
}

impl TreeColumnAccumulatorP1 {
    fn new() -> Self {
        Self {
            tallest: -1,
            visible_from_top: 0,
            visible_from_below_only: [false; 10],
        }
    }

    fn count(&self) -> usize {
        self.visible_from_top + self.visible_from_below_only.iter().filter(|&&x| x).count()
    }
}

fn count_visible(forest: &str) -> usize {
    let mut col_acc = Vec::new();
    col_acc.resize(
        forest.lines().next().unwrap().len(),
        TreeColumnAccumulatorP1::new(),
    );
    let mut visible_from_sides_count = 0;
    let mut sidevis = SideVisibility::new();
    for ln in forest.lines() {
        sidevis.update_from_line(ln);
        visible_from_sides_count += sidevis.count();
        for ((height, acc), visible_from_side) in ln
            .chars()
            .map(|ch| ch.to_digit(10).expect("Got a non-digit?") as i32)
            .zip(col_acc.iter_mut())
            .zip(sidevis.0.iter())
        {
            for h in 0..=height {
                acc.visible_from_below_only[h as usize] = false;
            }
            if !visible_from_side {
                if acc.tallest < height {
                    acc.visible_from_top = acc.visible_from_top + 1;
                } else {
                    acc.visible_from_below_only[height as usize] = true;
                }
            }
            acc.tallest = std::cmp::max(acc.tallest, height);
        }
    }
    visible_from_sides_count
        + col_acc
            .iter()
            .map(TreeColumnAccumulatorP1::count)
            .sum::<usize>()
}

#[cfg(test)]
mod tests_p1 {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            count_visible(
                "30373\n\
                 25512\n\
                 65332\n\
                 33549\n\
                 35390\n"
            ),
            21
        );
    }

    #[test]
    fn test_visible_from_below() {
        assert_eq!(
            count_visible(
                "30373\n\
                 25512\n\
                 65332\n\
                 33349\n\
                 35290\n"
            ),
            22
        );
    }

    #[test]
    fn test_visible_from_below_and_side() {
        assert_eq!(
            count_visible(
                "30373\n\
                 25512\n\
                 65332\n\
                 33459\n\
                 35290\n"
            ),
            22
        );
    }

    #[test]
    fn test_visible_variios() {
        assert_eq!(
            count_visible(
                "00000\n\
                 00000\n\
                 00000\n\
                 00000\n\
                 00000\n"
            ),
            16
        );
        assert_eq!(
            count_visible(
                "10000\n\
                 10000\n\
                 10000\n\
                 10000\n\
                 10000\n"
            ),
            16
        );
        assert_eq!(
            count_visible(
                "12000\n\
                 12000\n\
                 12000\n\
                 12000\n\
                 12000\n"
            ),
            19
        );
        assert_eq!(
            count_visible(
                "12000\n\
                 13300\n\
                 13300\n\
                 13300\n\
                 12000\n"
            ),
            22
        );
        assert_eq!(
            count_visible(
                "12000\n\
                 13300\n\
                 13300\n\
                 13322\n\
                 12022\n"
            ),
            23
        );
        assert_eq!(
            count_visible(
                "555959\n\
                 555969\n\
                 555969\n\
                 555989\n\
                 555999\n"
            ),
            23
        );
    }
}

fn find_most_scenic(forest: &str) -> usize {
    let width = forest.lines().next().unwrap().len();
    let forest: Vec<u8> = forest
        .chars()
        .filter(|&ch| ch != '\n')
        .map(|ch| ch.to_digit(10).expect("Got a non-digit?") as u8)
        .collect();
    let left_up_scenic_scores = partial_scenic_score(forest.iter().cloned(), width);
    let right_down_scenic_scores_rev =
        partial_scenic_score(forest.iter().rev().cloned(), width);
    left_up_scenic_scores
        .iter()
        .zip(right_down_scenic_scores_rev.iter().rev())
        .fold(0, |acc, (lu, rd)| std::cmp::max(acc, lu * rd))
}

fn partial_scenic_score<I>(iter: I, width: usize) -> Vec<usize>
where
    I: Iterator<Item = u8>,
{
    let mut result = vec![];
    result.reserve(iter.size_hint().0);
    let mut last_row_for_height = vec![[0; 10]; width];
    result.resize(width, 0); // 1st row
    for (row_idx, row) in iter.chunks(width).into_iter().enumerate().skip(1) {
        let mut last_col_for_height = [0; 10];
        result.push(0); // 1st col
        for (col_idx, (t, last_row_for_height)) in
            row.zip(last_row_for_height.iter_mut()).enumerate().skip(1)
        {
            let mut score = 1;
            if row_idx > 0 {
                score *= row_idx - last_row_for_height[t as usize];
            }
            if col_idx > 0 {
                score *= col_idx - last_col_for_height[t as usize];
            }
            for h in 0..=t {
                last_col_for_height[h as usize] = col_idx;
            }
            for h in 0..=t {
                last_row_for_height[h as usize] = row_idx;
            }
            result.push(score);
        }
    }
    result
}

#[cfg(test)]
mod tests_p2 {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            find_most_scenic(
                "30373\n\
                 25512\n\
                 65332\n\
                 33549\n\
                 35390\n"
            ),
            8
        );
    }

    #[test]
    fn test_various() {
        assert_eq!(
            find_most_scenic(
                "11111\n\
                 11111\n\
                 11111\n\
                 11111\n\
                 11111\n"
            ),
            1
        );
        assert_eq!(
            find_most_scenic(
                "11111\n\
                 11111\n\
                 11211\n\
                 11111\n\
                 11111\n"
            ),
            16
        );
        assert_eq!(
            find_most_scenic(
                "11111\n\
                 11111\n\
                 13231\n\
                 11111\n\
                 11111\n"
            ),
            8
        );
        assert_eq!(
            find_most_scenic(
                "11111\n\
                 77777\n\
                 13231\n\
                 11111\n\
                 11111\n"
            ),
            4
        );
        assert_eq!(
            find_most_scenic(
                "11111\n\
                 22222\n\
                 33333\n\
                 22222\n\
                 11111\n"
            ),
            4
        );
        assert_eq!(
            find_most_scenic(
                "98989\n\
                 88888\n\
                 98789\n\
                 88888\n\
                 98989\n"
            ),
            2
        );
        assert_eq!(
            find_most_scenic(
                "12345\n\
                 23456\n\
                 34567\n\
                 45678\n\
                 56789\n"
            ),
            9
        );
        assert_eq!(
            find_most_scenic(
                "999999999\n\
                 900000009\n\
                 999999999\n"
            ),
            1
        );
        assert_eq!(
            find_most_scenic(
                "999999999\n\
                 900010009\n\
                 999999999\n"
            ),
            16
        );
        assert_eq!(
            find_most_scenic(
                "999999999\n\
                 900001009\n\
                 999999999\n"
            ),
            15
        );
    }
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("07/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let count = count_visible(&input);
    println!("Number of visible trees: {count}");
    let most_scenic = find_most_scenic(&input);
    println!("Highest scenic score: {most_scenic}");
}
