use std::str::FromStr;

use anyhow::{anyhow, Context};
use common::twod::{Grid, Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Air,
    Rock,
    Sand,
    SandSource,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
            Cell::SandSource => '+',
        }
    }
}

pub struct Cave {
    grid: Grid<Cell>,
    x_offset: i32,
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
            data: vec![Cell::Air; grid_size],
            width,
        };
        for path in paths.iter() {
            let mut offset_path = path.clone();
            offset_path.iter_mut().for_each(|p| p.x -= x_offset);
            grid.fill_path(&offset_path, Cell::Rock);
        }
        grid[Point { x: 500 - x_offset, y: 0 }] = Cell::SandSource;
        Ok(Cave { grid, x_offset })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_cave() {
        let cave: Cave = "498,4 -> 498,6 -> 496,6\n\
                          503,4 -> 502,4 -> 502,9 -> 494,9"
            .parse()
            .unwrap();
        println!("{}", cave.x_offset);
        println!("{}", cave.to_string());
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
    }
}
