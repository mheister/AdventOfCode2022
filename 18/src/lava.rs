use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Cube {
    pub fn neighbours(&self) -> CubeNeighbours {
        CubeNeighbours { cube: *self, i: 0 }
    }
}

pub struct CubeNeighbours {
    cube: Cube,
    i: u8,
}

impl Iterator for CubeNeighbours {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 6 {
            return None;
        }
        let mut result = self.cube;
        let coord =
            &mut [&mut result.x, &mut result.y, &mut result.z][(self.i % 3) as usize];
        let off = [-1, 1][(self.i / 3) as usize];
        **coord += off;
        self.i += 1;
        Some(result)
    }
}

pub struct Droplet(Vec<Cube>);

impl FromStr for Droplet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Droplet(
            s.lines()
                .map(|ln| -> Result<Cube, Self::Err> {
                    let (x, yz) = ln.split_once(',').ok_or(anyhow!("Expected ','"))?;
                    let (y, z) = yz.split_once(',').ok_or(anyhow!("Expected ','"))?;
                    Ok(Cube {
                        x: x.parse()?,
                        y: y.parse()?,
                        z: z.parse()?,
                    })
                })
                .collect::<Result<Vec<Cube>, _>>()?,
        ))
    }
}

impl Droplet {
    pub fn surface_area(&self) -> usize {
        let cubes_map = self.0.iter().collect::<HashSet<_>>();
        let mut surface = 0;
        for cube in self.0.iter() {
            surface += 6;
            for n in cube.neighbours() {
                if cubes_map.contains(&n) {
                    surface -= 1;
                }
            }
        }
        surface
    }

    fn is_outside(cube: &Cube) -> bool {
        // All coordinates are between 0 and 20, non-inclusive
        const MIN: i32 = 0;
        const MAX: i32 = 20;
        cube.x == MIN
            || cube.x >= MAX
            || cube.y == MIN
            || cube.y >= MAX
            || cube.z == MIN
            || cube.z >= MAX
    }

    pub fn exterior_surface_area(&self) -> usize {
        let cubes_map = self.0.iter().cloned().collect::<HashSet<_>>();
        let mut air_cubes = HashSet::<Cube>::new();
        let mut surface = 0;
        for cube in self.0.iter() {
            surface += 6;
            for n in cube.neighbours() {
                if cubes_map.contains(&n) {
                    surface -= 1;
                } else {
                    air_cubes.insert(n);
                }
            }
        }
        while !air_cubes.is_empty() {
            let starting_air_cube = air_cubes.iter().next().unwrap().clone();
            air_cubes.remove(&starting_air_cube);
            let mut air_pocket = vec![starting_air_cube];
            let mut air_pocket_map = air_pocket.iter().cloned().collect::<HashSet<_>>();
            let mut pidx = 0;
            loop {
                if pidx >= air_pocket.len() {
                    surface -= Droplet(air_pocket).surface_area();
                    break;
                }
                let mut outside = false;
                for n in air_pocket.get(pidx).unwrap().neighbours() {
                    air_cubes.remove(&n);
                    if cubes_map.contains(&n) {
                        continue; // Lava
                    }
                    if air_pocket_map.contains(&n) {
                        continue; // Already visited
                    }
                    if Droplet::is_outside(&n) {
                        // No air pocket
                        outside = true;
                        break;
                    }
                    air_pocket.push(n);
                    air_pocket_map.insert(n);
                }
                if outside {
                    break;
                }
                pidx += 1;
            }
        }
        surface
    }
}
