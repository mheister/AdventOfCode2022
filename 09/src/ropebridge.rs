use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    L,
    U,
    R,
    D,
}

pub struct RopeBridge {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl RopeBridge {
    pub fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            visited: HashSet::from([(0, 0)]),
        }
    }

    pub fn head(&self) -> (i32, i32) {
        self.head
    }

    pub fn tail(&self) -> (i32, i32) {
        self.tail
    }

    pub fn motion(&mut self, dir: Direction, count: usize) {
        let step = match dir {
            Direction::L => (-1, 0),
            Direction::U => (0, 1),
            Direction::R => (1, 0),
            Direction::D => (0, -1),
        };
        for _ in 0..count {
            self.head = (self.head.0 + step.0, self.head.1 + step.1);
            self.tail_motion();
        }
    }

    pub fn count_visited_positions(&self) -> usize {
        self.visited.len()
    }

    fn tail_motion(&mut self) {
        let dx = self.head.0 - self.tail.0;
        let dy = self.head.1 - self.tail.1;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.tail.0 += dx.signum();
            self.tail.1 += dy.signum();
            self.visited.insert(self.tail);
        }
    }
}

#[cfg(test)]
mod ropebridge_tests {
    use super::*;

    #[test]
    fn new() {
        let b = RopeBridge::new();
        assert_eq!(b.head(), (0, 0));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }

    #[test]
    fn vertical_motion() {
        let mut b = RopeBridge::new();
        b.motion(Direction::R, 1);
        assert_eq!(b.head(), (1, 0));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
        b.motion(Direction::L, 2);
        assert_eq!(b.head(), (-1, 0));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }

    #[test]
    fn horizontal_motion() {
        let mut b = RopeBridge::new();
        b.motion(Direction::U, 1);
        assert_eq!(b.head(), (0, 1));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
        b.motion(Direction::D, 2);
        assert_eq!(b.head(), (0, -1));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }

    #[test]
    fn tail_should_follow_when_two_steps_away() {
        let mut b = RopeBridge::new();
        b.motion(Direction::R, 2);
        assert_eq!(b.head(), (2, 0));
        assert_eq!(b.tail(), (1, 0));
        assert_eq!(b.count_visited_positions(), 2);
    }

    #[test]
    fn tail_should_not_follow_when_touching_diagonally() {
        let mut b = RopeBridge::new();
        b.motion(Direction::R, 1);
        b.motion(Direction::U, 1);
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }
}
