use common::twod::Point;
use std::{fmt, str::FromStr};

const CHAMBER_WIDTH: usize = 7;
const MAX_ROCK_SHAPE_HEIGHT: usize = 4;
// Init value puts a 'wall' at the right hand side for collision detection
const GRID_ROW_INITVAL: u8 = 1 << 7;

/// Rock shapes as bit mask (one u8 per line), in different horizontal positions (shape[2]
/// would be the initial position)
type RockShape = [[u8; MAX_ROCK_SHAPE_HEIGHT]; CHAMBER_WIDTH + 1];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Clone)]
pub struct JetPattern(Vec<Jet>);

pub struct Chamber {
    grid: Vec<u8>,
    base_y: usize, // height from which we store data in grid
    high_point: i32,
    rock_shapes: Vec<RockShape>,
    next_rock_shape_idx: usize,
    jet_pattern: Vec<Jet>,
    next_jet_idx: usize,
}

impl Chamber {
    pub fn new(jet_pattern: JetPattern) -> Self {
        let rock_shapes = get_rock_shapes();
        Self {
            grid: vec![GRID_ROW_INITVAL; 5000],
            base_y: 0,
            rock_shapes,
            high_point: -1,
            next_rock_shape_idx: 0,
            jet_pattern: jet_pattern.0,
            next_jet_idx: 0,
        }
    }

    pub fn tower_height(&self) -> usize {
        self.base_y + (self.high_point + 1) as usize
    }

    pub fn rumble(&mut self, n_rocks: usize) {
        for i in 0..n_rocks {
            self.drop_rock();
            if i % 100_000_000 == 0 {
                println!("(INFO) {}M rocks dropped", i / 1_000_000);
            }
        }
    }

    fn drop_rock(&mut self) {
        let rock = self
            .rock_shapes
            .get(self.next_rock_shape_idx)
            .unwrap()
            .clone();
        self.next_rock_shape_idx = (self.next_rock_shape_idx + 1) % self.rock_shapes.len();
        let req_height = self.tower_height() + 4 + MAX_ROCK_SHAPE_HEIGHT;
        if self.grid.len() < req_height {
            self.grid.resize(req_height, GRID_ROW_INITVAL);
        }
        let mut anchor = Point {
            x: 2,
            y: self.high_point + 4,
        };
        loop {
            let jet = self.jet_pattern.get(self.next_jet_idx).unwrap().clone();
            self.next_jet_idx = (self.next_jet_idx + 1) % self.jet_pattern.len();
            let anchor_pushed = match jet {
                Jet::Left => Point {
                    x: anchor.x - 1,
                    ..anchor
                },
                Jet::Right => Point {
                    x: anchor.x + 1,
                    ..anchor
                },
            };
            if !self.collision(&rock, anchor_pushed) {
                anchor = anchor_pushed;
            }
            let anchor_fallen = Point {
                y: anchor.y - 1,
                ..anchor
            };
            if self.collision(&rock, anchor_fallen) {
                break;
            } else {
                anchor = anchor_fallen;
            }
        }
        self.come_to_rest(&rock, anchor);
    }

    fn collision(&self, rock: &RockShape, anchor: Point) -> bool {
        if anchor.x < 0 || anchor.y < 0 {
            return true;
        }
        let rock = rock[anchor.x as usize]; // x-translated shape
        self.grid[anchor.y as usize..anchor.y as usize + MAX_ROCK_SHAPE_HEIGHT]
            .iter()
            .zip(rock.iter())
            .map(|(&a, &b)| a & b)
            .any(|r| r != 0)
    }

    fn come_to_rest(&mut self, rock: &RockShape, anchor: Point) {
        let rock = rock[anchor.x as usize]; // x-translated shape
        self.grid[anchor.y as usize..anchor.y as usize + MAX_ROCK_SHAPE_HEIGHT]
            .iter_mut()
            .zip(rock.iter())
            .for_each(|(a, &b)| *a |= b);
        self.high_point = (std::cmp::max(0, self.high_point)..self.grid.len() as i32)
            .find(|&v| self.grid[v as usize] & 0b1111111 == 0)
            .unwrap_or(self.grid.len() as i32)
            - 1;
        self.adjust_view();
    }

    fn adjust_view(&mut self) {
        const MAX_CHAMBER_VIEW_HEIGHT: i32 = 300_000;
        if self.high_point <= MAX_CHAMBER_VIEW_HEIGHT {
            return;
        }
        let rows_to_keep = self
            .grid
            .iter()
            .rev()
            .enumerate()
            .skip(self.grid.len() - (self.high_point + 1) as usize)
            .scan(0u8, |acc, (n, row)| {
                *acc |= *row;
                if *acc == 0b11111111 {
                    None
                } else {
                    Some(n)
                }
            })
            .last()
            .unwrap_or(0)
            + 10;
        if rows_to_keep >= self.grid.len() {
            return;
        }
        let rows_to_cut = self.grid.len() - rows_to_keep;
        self.base_y += rows_to_cut;
        self.high_point -= rows_to_cut as i32;
        self.grid.drain(0..rows_to_cut);
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = (self.high_point + 2) as usize;
        self.grid
            .iter()
            .take(height)
            .rev()
            .map(|&row| {
                (0..CHAMBER_WIDTH)
                    .map(|bit_idx| row & (1u8 << bit_idx) != 0)
                    .map(|rock| match rock {
                        false => '.',
                        true => '#',
                    })
                    .map(|ch| write!(f, "{ch}"))
                    .collect::<fmt::Result>()
                    .and_then(|()| write!(f, "\n"))
            })
            .collect()
    }
}

fn to_string(s: RockShape) -> String {
    let mut result = "".to_owned();
    for row in (0..MAX_ROCK_SHAPE_HEIGHT).rev() {
        for shape in s {
            result += &(0..CHAMBER_WIDTH)
                .map(|bit_idx| shape[row] & (1u8 << bit_idx) != 0)
                .map(|rock| match rock {
                    false => '.',
                    true => '#',
                })
                .collect::<String>();
            result += "  |  ";
        }
        result += "\n";
    }
    result
}

#[allow(dead_code)]
pub fn print_shapes() {
    let shapes = get_rock_shapes();
    for sh in shapes {
        println!("{}\n\n", to_string(sh));
    }
}

fn get_rock_shapes() -> Vec<RockShape> {
    let base_shapes = vec![
        [
            0b00001111u8, //
            0b00000000u8, //
            0b00000000u8, //
            0b00000000u8, //
        ],
        [
            0b00000010u8, //
            0b00000111u8, //
            0b00000010u8, //
            0b00000000u8, //
        ],
        [
            0b00000111u8, //
            0b00000100u8, //
            0b00000100u8, //
            0b00000000u8, //
        ],
        [
            0b00000001u8, //
            0b00000001u8, //
            0b00000001u8, //
            0b00000001u8, //
        ],
        [
            0b00000011u8, //
            0b00000011u8, //
            0b00000000u8, //
            0b00000000u8, //
        ],
    ];
    base_shapes
        .iter()
        .map(|base| {
            let mut translation_set = [*base; CHAMBER_WIDTH + 1];
            for t in 1..CHAMBER_WIDTH + 1 {
                    translation_set[t].iter_mut().for_each(|row| {
                        *row <<= t;
                    })
            }
            translation_set
        })
        .collect()
}

impl FromStr for JetPattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let jets = s
            .trim()
            .chars()
            .map(|ch| match ch {
                '<' => Ok(Jet::Left),
                '>' => Ok(Jet::Right),
                other => Err(anyhow::anyhow!("Unrecognized character '{other}'")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(JetPattern(jets))
    }
}

#[cfg(test)]
mod leftwind {
    use super::*;

    #[test]
    fn after_first_drop_pile_should_be_one_tall() {
        let jets = "<".parse().unwrap();
        let mut chamber = Chamber::new(jets);
        chamber.rumble(1);
        assert_eq!(chamber.tower_height(), 1);
    }

    #[test]
    fn after_second_drop_pile_should_be_four_tall() {
        let jets = "<".parse().unwrap();
        let mut chamber = Chamber::new(jets);
        chamber.rumble(2);
        assert_eq!(chamber.tower_height(), 4);
    }

    #[test]
    fn after_third_drop_pile_should_be_seven_tall() {
        let jets = "<".parse().unwrap();
        let mut chamber = Chamber::new(jets);
        chamber.rumble(3);
        assert_eq!(chamber.tower_height(), 7);
    }

    #[test]
    fn after_fourth_drop_pile_should_be_nine_tall() {
        let jets = "<".parse().unwrap();
        let mut chamber = Chamber::new(jets);
        chamber.rumble(4);
        assert_eq!(chamber.tower_height(), 9);
    }
}
