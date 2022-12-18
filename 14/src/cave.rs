use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::twod::{Grid, Point};

const SAND_SOURCE_X: i32 = 500;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Air,
    Rock,
    Sand,
    SandSource,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::SandSource => '+',
        }
    }
}

pub struct Cave {
    grid: Grid<Tile>,
    x_offset: i32,
    y_max: i32, // below y_max, sand falls into the void
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths = s
            .lines()
            .map(|ln| {
                ln.split(" -> ")
                    .map(|point_str| -> anyhow::Result<Point> {
                        let (x, y) = point_str
                            .split_once(',')
                            .ok_or(anyhow!("Could not parse point '{point_str}'"))?;
                        Ok(Point {
                            x: x.parse().context(
                                "Failed to parse x coordinate in '{point_str}'",
                            )?,
                            y: y.parse().context(
                                "Failed to parse y coordinate in '{point_str}'",
                            )?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("Failed to parse path {ln}")
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let x_min = paths.iter().flatten().map(|p| p.x).min().unwrap_or(0);
        let x_max = paths.iter().flatten().map(|p| p.x).max().unwrap_or(0);
        let y_max = paths.iter().flatten().map(|p| p.y).max().unwrap_or(0);
        let x_offset = x_min;
        let width = (x_max - x_min + 1) as usize;
        let grid_size = width * (y_max + 1) as usize;
        let mut grid = Grid {
            data: vec![Tile::Air; grid_size],
            width,
        };
        for path in paths.iter() {
            let mut offset_path = path.clone();
            offset_path.iter_mut().for_each(|p| p.x -= x_offset);
            grid.fill_path(&offset_path, Tile::Rock);
        }
        grid[Point {
            x: SAND_SOURCE_X - x_offset,
            y: 0,
        }] = Tile::SandSource;
        Ok(Cave {
            grid,
            x_offset,
            y_max,
        })
    }
}

impl ToString for Cave {
    fn to_string(&self) -> String {
        self.grid
            .data
            .chunks(self.grid.width)
            .enumerate()
            .map(|(y, ln)| {
                let line = ln.iter().cloned().map(char::from).collect::<String>();
                format!("{:04} {}", y, line)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DropSandResult {
    Overflow,
    SandRests,
}

impl Cave {
    fn drop_sand(&mut self, p: Point) -> DropSandResult {
        let mut p = p;
        if self.grid[p] == Tile::Sand {
            return DropSandResult::Overflow;
        }
        let mut falling = true;
        while falling {
            falling = false;
            for dest in [
                Point { x: p.x, y: p.y + 1 },
                Point {
                    x: p.x - 1,
                    y: p.y + 1,
                },
                Point {
                    x: p.x + 1,
                    y: p.y + 1,
                },
            ] {
                if dest.x < 0 || dest.x >= self.grid.width as i32 || dest.y > self.y_max {
                    return DropSandResult::Overflow;
                } else if self.grid[dest] == Tile::Air {
                    p = dest;
                    falling = true;
                    break;
                }
            }
        }
        self.grid[p] = Tile::Sand;
        return DropSandResult::SandRests;
    }

    pub fn fill_sand(&mut self) {
        let spawn_point = Point {
            x: SAND_SOURCE_X - self.x_offset,
            y: 1,
        };
        loop {
            let res = self.drop_sand(spawn_point);
            if res == DropSandResult::Overflow {
                return;
            }
        }
    }

    pub fn count_sand(&self) -> usize {
        self.grid.data.iter().cloned().filter(|&tile| tile == Tile::Sand).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_sample_cave() {
        let cave: Cave = "498,4 -> 498,6 -> 496,6\n\
                          503,4 -> 502,4 -> 502,9 -> 494,9"
            .parse()
            .unwrap();
        assert_eq!(
            cave.to_string(),
            "0000 ......+...\n\
             0001 ..........\n\
             0002 ..........\n\
             0003 ..........\n\
             0004 ....#...##\n\
             0005 ....#...#.\n\
             0006 ..###...#.\n\
             0007 ........#.\n\
             0008 ........#.\n\
             0009 #########."
        );
        assert_eq!(cave.count_sand(), 0);
    }

    #[test]
    fn fill_sample_cave() {
        let mut cave: Cave = "498,4 -> 498,6 -> 496,6\n\
                              503,4 -> 502,4 -> 502,9 -> 494,9"
            .parse()
            .unwrap();
        cave.fill_sand();
        assert_eq!(
            cave.to_string(),
            "0000 ......+...\n\
             0001 ..........\n\
             0002 ......o...\n\
             0003 .....ooo..\n\
             0004 ....#ooo##\n\
             0005 ...o#ooo#.\n\
             0006 ..###ooo#.\n\
             0007 ....oooo#.\n\
             0008 .o.ooooo#.\n\
             0009 #########."
        );
        assert_eq!(cave.count_sand(), 24);
    }
}
