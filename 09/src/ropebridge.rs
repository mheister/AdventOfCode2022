use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    L,
    U,
    R,
    D,
}

pub struct RopeBridge<const L: usize> {
    rope: [(i32, i32); L],
    visited: HashSet<(i32, i32)>,
}

impl<const L: usize> RopeBridge<L> {
    pub fn new() -> Self {
        Self {
            rope: [(0, 0); L],
            visited: HashSet::from([(0, 0)]),
        }
    }

    pub fn head(&self) -> (i32, i32) {
        self.rope.first().cloned().unwrap_or((0, 0))
    }

    pub fn tail(&self) -> (i32, i32) {
        self.rope.last().cloned().unwrap_or((0, 0))
    }

    pub fn motion(&mut self, dir: Direction, count: usize) {
        let step = match dir {
            Direction::L => (-1, 0),
            Direction::U => (0, 1),
            Direction::R => (1, 0),
            Direction::D => (0, -1),
        };
        for _ in 0..count {
            if let Some(head) = self.rope.first_mut() {
                head.0 += step.0;
                head.1 += step.1;
            }
            self.relax_rope();
        }
    }

    pub fn count_visited_positions(&self) -> usize {
        self.visited.len()
    }

    fn relax_rope(&mut self) {
        let head = self.rope.first().unwrap().clone();
        for knot in self.rope.iter_mut().skip(1) {
            let dx = head.0 - knot.0;
            let dy = head.1 - knot.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                knot.0 += dx.signum();
                knot.1 += dy.signum();
            }
        }
        if let Some(tail) = self.rope.last() {
            self.visited.insert(*tail);
        }
    }
}

#[cfg(test)]
mod ropebridge_tests {
    use super::*;

    #[test]
    fn new() {
        let b = RopeBridge::<2>::new();
        assert_eq!(b.head(), (0, 0));
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }

    #[test]
    fn vertical_motion() {
        let mut b = RopeBridge::<2>::new();
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
        let mut b = RopeBridge::<2>::new();
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
        let mut b = RopeBridge::<2>::new();
        b.motion(Direction::R, 2);
        assert_eq!(b.head(), (2, 0));
        assert_eq!(b.tail(), (1, 0));
        assert_eq!(b.count_visited_positions(), 2);
    }

    #[test]
    fn tail_should_not_follow_when_touching_diagonally() {
        let mut b = RopeBridge::<2>::new();
        b.motion(Direction::R, 1);
        b.motion(Direction::U, 1);
        assert_eq!(b.tail(), (0, 0));
        assert_eq!(b.count_visited_positions(), 1);
    }
}
