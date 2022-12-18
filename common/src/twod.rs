#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn neighbours(&self, width: i32, height: i32) -> PointNeighbours {
        PointNeighbours {
            x: self.x,
            y: self.y,
            x_lim: width,
            y_lim: height,
            i: 0,
        }
    }
}

pub struct PointNeighbours {
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
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
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

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn fill_path(&mut self, path: &[Point], item: T) {
        for segment in path.windows(2) {
            self.fill_line(*segment.get(0).unwrap(), *segment.get(1).unwrap(), item);
        }
    }
    pub fn fill_line(&mut self, p1: Point, p2: Point, item: T) {
        let mut p = p1;
        while p != p2 {
            self[p] = item;
            let (step_x, step_y) = ((p2.x - p1.x).signum(), (p2.y - p1.y).signum());
            p = Point { x: p.x + step_x, y: p.y + step_y }
        }
        self[p2] = item;
    }
}
