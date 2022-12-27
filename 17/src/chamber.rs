use std::{fmt, str::FromStr};

use common::twod::{Grid, Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
}

type RockShape = Grid<Tile>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Clone)]
pub struct JetPattern(Vec<Jet>);

pub struct Chamber {
    grid: Grid<Tile>,
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
            grid: Grid {
                data: vec![Tile::Air; 7 * 5000],
                width: 7,
            },
            rock_shapes,
            high_point: -1,
            next_rock_shape_idx: 0,
            jet_pattern: jet_pattern.0,
            next_jet_idx: 0,
        }
    }

    pub fn tower_height(&self) -> usize {
        (self.high_point + 1).try_into().unwrap()
    }

    pub fn rumble(&mut self, n_rocks: usize) {
        for _ in 0..n_rocks {
            self.drop_rock();
        }
    }

    fn drop_rock(&mut self) {
        let rock = self
            .rock_shapes
            .get(self.next_rock_shape_idx)
            .unwrap()
            .clone();
        self.next_rock_shape_idx = (self.next_rock_shape_idx + 1) % self.rock_shapes.len();
        self.grid
            .ensure_height(self.tower_height() + 4 + rock.height(), Tile::Air);
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
        self.come_to_rest(rock, anchor);
    }

    fn collision(&self, rock: &Grid<Tile>, anchor: Point) -> bool {
        for y in 0..rock.height() as i32 {
            for x in 0..rock.width() as i32 {
                if rock[Point { x, y }] == Tile::Air {
                    continue;
                }
                let grid_point = Point {
                    x: x + anchor.x,
                    y: y + anchor.y,
                };
                if grid_point.x < 0
                    || grid_point.x >= self.grid.width as i32
                    || grid_point.y < 0
                    || self.grid[grid_point] != Tile::Air
                {
                    return true;
                }
            }
        }
        false
    }

    fn come_to_rest(&mut self, rock: Grid<Tile>, anchor: Point) {
        for y in 0..rock.height() as i32 {
            for x in 0..rock.width() as i32 {
                if rock[Point { x, y }] == Tile::Air {
                    continue;
                }
                let grid_point = Point {
                    x: x + anchor.x,
                    y: y + anchor.y,
                };
                let grid_tile = &mut self.grid[grid_point];
                if *grid_tile == Tile::Air {
                    *grid_tile = Tile::Rock;
                    self.high_point = std::cmp::max(self.high_point, grid_point.y);
                }
            }
        }
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.tower_height() + 1;
        self.grid
            .data
            .chunks(self.grid.width)
            .take(height)
            .rev()
            .map(|ln| {
                ln.iter()
                    .map(|tile| match tile {
                        Tile::Air => '.',
                        Tile::Rock => '#',
                    })
                    .map(|ch| write!(f, "{ch}"))
                    .collect::<fmt::Result>()
                    .and_then(|()| write!(f, "\n"))
            })
            .collect()
    }
}

fn get_rock_shapes() -> Vec<RockShape> {
    const INP: &str = "####\n\
                       \n\
                       .#.\n\
                       ###\n\
                       .#.\n\
                       \n\
                       ..#\n\
                       ..#\n\
                       ###\n\
                       \n\
                       #\n\
                       #\n\
                       #\n\
                       #\n\
                       \n\
                       ##\n\
                       ##";
    INP.split("\n\n")
        .map(|shape_str| {
            let width = shape_str.lines().next().unwrap().len();
            let data = shape_str
                .lines()
                .rev()
                .flat_map(|ln| ln.chars())
                .map(|c| match c {
                    '#' => Tile::Rock,
                    _ => Tile::Air,
                })
                .collect();
            RockShape { width, data }
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
