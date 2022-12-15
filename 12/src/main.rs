use std::{collections::HashSet, env, fs, str::FromStr};
use anyhow::Context;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbours(&self, width: i32, height: i32) -> PointNeighbours {
        PointNeighbours {
            x: self.x,
            y: self.y,
            x_lim: width,
            y_lim: height,
            i: 0,
        }
    }
}

struct PointNeighbours {
    x: i32,
    y: i32,
    x_lim: i32,
    y_lim: i32,
    i: u8,
}

impl Iterator for PointNeighbours {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut point;
        loop {
            if self.i as usize >= OFFSETS.len() {
                return None;
            }
            let off = OFFSETS[self.i as usize];
            self.i += 1;
            point = Point {
                x: self.x + off.0,
                y: self.y + off.1,
            };
            if point.x >= 0 && point.y >= 0 && point.x < self.x_lim && point.y < self.y_lim
            {
                break;
            }
        }
        Some(point)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, p: Point) -> &Self::Output {
        &self.data[p.x as usize + self.width * p.y as usize]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self.data[p.x as usize + self.width * p.y as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field {
    heightmap: Grid<u8>,
    start: Point,
    end: Point,
}


impl Field {
    fn find_shortest_path(&self) -> Result<i32, anyhow::Error> {
        let mut map = Grid::<i32> {
            data: vec![i32::MAX; self.heightmap.data.len()],
            width: self.heightmap.width,
        };
        map[self.start] = 0;
        let height = map.data.len() / map.width;
        let mut worklist = HashSet::new();
        worklist.insert(self.start);
        while !worklist.is_empty() {
            let point = worklist.iter().cloned().next().unwrap();
            worklist.remove(&point);
            point
                .neighbours(map.width as i32, height as i32)
                .filter(|&n| {
                    (self.heightmap[n] as i32 - self.heightmap[point] as i32) <= 1
                })
                .for_each(|n| {
                    let dist_via_point = map[point] + 1;
                    let known_dist_to_n = &mut map[n];
                    if *known_dist_to_n > dist_via_point {
                        // println!("To {},{}:{} with {dist_via_point}", n.x, n.y, self.heightmap[n] as char);
                        *known_dist_to_n = dist_via_point;
                        worklist.insert(n);
                    }
                });
        }
        let target_dist = map[self.end];
        Ok(target_dist)
    }
    fn find_shortest_path_from_any_a(&self) -> Result<i32, anyhow::Error> {
        let mut map = Grid::<i32> {
            data: vec![i32::MAX; self.heightmap.data.len()],
            width: self.heightmap.width,
        };
        map[self.end] = 0;
        let height = map.data.len() / map.width;
        let mut worklist = HashSet::new();
        worklist.insert(self.end);
        let mut a_points = vec![];
        while !worklist.is_empty() {
            let point = worklist.iter().cloned().next().unwrap();
            worklist.remove(&point);
            point
                .neighbours(map.width as i32, height as i32)
                .filter(|&n| {
                    (self.heightmap[point] as i32 - self.heightmap[n] as i32) <= 1
                })
                .for_each(|n| {
                    let dist_via_point = map[point] + 1;
                    let known_dist_to_n = &mut map[n];
                    if *known_dist_to_n > dist_via_point {
                        if self.heightmap[n] == 'a' as u8 {
                            a_points.push(dist_via_point);
                        }
                        // println!("To {},{}:{} with {dist_via_point}", n.x, n.y, self.heightmap[n] as char);
                        *known_dist_to_n = dist_via_point;
                        worklist.insert(n);
                    }
                });
        }
        if let Some(&d) = a_points.iter().min() {
            return Ok(d);
        }

        Err(anyhow::anyhow!("Somehow we did not find any square of elevation a from which E is reachable"))
    }
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heightmap = Grid::<u8> {
            data: s.bytes().filter(|&b| b != '\n' as u8).collect(),
            width: s.lines().next().map(|l| l.len()).unwrap_or(0usize),
        };
        let flat_idx_to_point = |idx: usize| Point {
            x: (idx % heightmap.width) as i32,
            y: (idx / heightmap.width) as i32,
        };
        let (start_flat_idx, start_ref) = heightmap
            .data
            .iter_mut()
            .enumerate()
            .find(|(_, &mut c)| c == 'S' as u8)
            .context("Could not find starting point")?;
        *start_ref = 'a' as u8;
        let start = flat_idx_to_point(start_flat_idx);
        let (end_flat_idx, start_ref) = heightmap
            .data
            .iter_mut()
            .enumerate()
            .find(|(_, &mut c)| c == 'E' as u8)
            .context("Could not find target point")?;
        *start_ref = 'z' as u8;
        let end = flat_idx_to_point(end_flat_idx);
        Ok(Self {
            heightmap,
            start,
            end,
        })
    }
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("12/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let field: Field = input.parse().unwrap();
    let dist  = field.find_shortest_path().unwrap();
    println!("Shortest path has {dist} steps");
    let dist_any_a  = field.find_shortest_path_from_any_a().unwrap();
    println!("Shortest path from any a square has {dist_any_a} steps");
}
