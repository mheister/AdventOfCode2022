use std::str::FromStr;

use anyhow::anyhow;

struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

struct Droplet(Vec<Cube>);

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
    fn count_sides(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let drop: Droplet = "1,1,1\n2,1,1".parse().unwrap();
        assert_eq!(drop.count_sides(), 10);
    }
}


fn main() {
    println!("Hello, world!");
}
