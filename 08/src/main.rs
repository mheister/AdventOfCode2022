#![allow(dead_code)]

use std::{env, fs};

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
struct TreeColumnAccumulator {
    // left_view_blocked: bool,
    tallest: i32,

    // number of trees visible from top and/or bottom but not from the sides
    visible_from_top: usize,

    // flags indicating that there is a tree of the given height visible exclusively
    // from below
    visible_from_below_only: [bool; 10],
}

impl TreeColumnAccumulator {
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

fn count_visible(field: &str) -> usize {
    let mut col_acc = Vec::new();
    col_acc.resize(
        field.lines().next().unwrap().len(),
        TreeColumnAccumulator::new(),
    );
    let mut visible_from_sides_count = 0;
    let mut sidevis = SideVisibility::new();
    for ln in field.lines() {
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
    dbg!(&visible_from_sides_count);
    visible_from_sides_count
        + col_acc
            .iter()
            .map(TreeColumnAccumulator::count)
            .sum::<usize>()
}

#[cfg(test)]
mod tests {
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

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("07/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let count = count_visible(&input);
    println!("Number of visible trees: {count}")
}
