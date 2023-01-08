use std::{env, fs};

mod lava;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cube_has_surface_of_6() {
        let drop: lava::Droplet = "1,1,1".parse().unwrap();
        assert_eq!(drop.surface_area(), 6);
    }

    #[test]
    fn simple_example() {
        let drop: lava::Droplet = "1,1,1\n2,1,1".parse().unwrap();
        assert_eq!(drop.surface_area(), 10);
    }

    #[test]
    fn larger_example() {
        let drop: lava::Droplet = "2,2,2\n\
                                   1,2,2\n\
                                   3,2,2\n\
                                   2,1,2\n\
                                   2,3,2\n\
                                   2,2,1\n\
                                   2,2,3\n\
                                   2,2,4\n\
                                   2,2,6\n\
                                   1,2,5\n\
                                   3,2,5\n\
                                   2,1,5\n\
                                   2,3,5"
            .parse()
            .unwrap();
        assert_eq!(drop.surface_area(), 64);
    }

    #[test]
    fn exterior_surface_larger_example() {
        let drop: lava::Droplet = "2,2,2\n\
                                   1,2,2\n\
                                   3,2,2\n\
                                   2,1,2\n\
                                   2,3,2\n\
                                   2,2,1\n\
                                   2,2,3\n\
                                   2,2,4\n\
                                   2,2,6\n\
                                   1,2,5\n\
                                   3,2,5\n\
                                   2,1,5\n\
                                   2,3,5"
            .parse()
            .unwrap();
        assert_eq!(drop.exterior_surface_area(), 58);
    }
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("18/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let drop: lava::Droplet = input.parse().unwrap();
    let surface = drop.surface_area();
    println!("Estimated surface area of lava droplet: {surface}");
    let exterior_surface = drop.exterior_surface_area();
    println!("Estimated exterior surface area of lava droplet: {exterior_surface}");
}
