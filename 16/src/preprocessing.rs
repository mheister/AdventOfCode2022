use std::ops::{BitOr, Deref, DerefMut, Index};

use crate::input;

pub type ValveIdx = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValveBitMask(pub u64);

impl Deref for ValveBitMask {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValveBitMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BitOr<u64> for ValveBitMask {
    type Output = ValveBitMask;

    fn bitor(self, rhs: u64) -> Self::Output {
        ValveBitMask(self.0 | rhs)
    }
}

impl FromIterator<ValveIdx> for ValveBitMask {
    fn from_iter<T: IntoIterator<Item = ValveIdx>>(iter: T) -> Self {
        iter.into_iter()
            .fold(ValveBitMask(0), |acc, idx| acc | (1u64 << idx))
    }
}

pub struct ValveIndices {
    mask: ValveBitMask,
    current: ValveIdx,
}

impl ValveBitMask {
    pub fn is_subset(self, other: Self) -> bool {
        !(self.0 & other.0) & self.0 == 0
    }

    pub fn is_superset(self, other: Self) -> bool {
        !(self.0 & other.0) & other.0 == 0
    }

    pub fn contains(self, pos: ValveIdx) -> bool {
        pos <= 64 && self.0 & (1u64 << pos) != 0
    }

    pub fn remove(&mut self, pos: ValveIdx) {
        self.0 &= !(1u64 << pos);
    }

    pub fn iter(&self) -> ValveIndices {
        ValveIndices {
            mask: self.clone(),
            current: 0,
        }
    }
}

impl Iterator for ValveIndices {
    type Item = ValveIdx;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.current..64).find(|&idx| self.mask.contains(idx)) {
            Some(idx) => {
                self.current = idx + 1;
                Some(idx)
            }
            None => {
                self.current = 65;
                None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    pub flow_rate: u32,
    pub tunnels: ValveBitMask,
}

#[derive(Debug)]
pub struct Cave {
    pub valves: Vec<Valve>,
    pub valve_labels: Vec<input::ValveLabel>,
}

impl From<&input::Cave> for Cave {
    fn from(input: &input::Cave) -> Self {
        let valve_labels: Vec<input::ValveLabel> = input.keys().cloned().collect();
        let valves = valve_labels
            .iter()
            .map(|label| {
                let valve = input.get(label).unwrap();
                let tunnels = valve
                    .tunnels
                    .iter()
                    .map(|label| {
                        valve_labels.iter().position(|l| *l == *label).unwrap() as ValveIdx
                    })
                    .collect();
                Valve {
                    flow_rate: valve.flow_rate,
                    tunnels,
                }
            })
            .collect();
        Cave {
            valves,
            valve_labels,
        }
    }
}

impl Index<ValveIdx> for Cave {
    type Output = Valve;

    fn index(&self, index: ValveIdx) -> &Self::Output {
        self.valves.index(index as usize)
    }
}

#[cfg(test)]
mod valve_bit_mask_tests {
    use super::ValveBitMask;

    #[test]
    fn subsets() {
        assert!(ValveBitMask(0b111).is_subset(ValveBitMask(0b111)));
        assert!(ValveBitMask(0b110).is_subset(ValveBitMask(0b111)));
        assert!(ValveBitMask(0b111).is_subset(ValveBitMask(0b1111)));
        assert!(!ValveBitMask(0b111).is_subset(ValveBitMask(0b110)));
    }

    #[test]
    fn supersets() {
        assert!(ValveBitMask(0b111).is_superset(ValveBitMask(0b111)));
        assert!(!ValveBitMask(0b110).is_superset(ValveBitMask(0b111)));
        assert!(ValveBitMask(0b1111).is_superset(ValveBitMask(0b111)));
        assert!(!ValveBitMask(0b111).is_superset(ValveBitMask(0b1111)));
        assert!(ValveBitMask(0b111).is_superset(ValveBitMask(0b110)));
    }

    #[test]
    fn contains() {
        assert!(ValveBitMask(0b111).contains(0));
        assert!(ValveBitMask(0b111).contains(1));
        assert!(ValveBitMask(0b111).contains(2));
        assert!(!ValveBitMask(0b111).contains(3));
        assert!(!ValveBitMask(0b111).contains(63));
        assert!(!ValveBitMask(0b111).contains(100));
    }

    #[test]
    fn remove() {
        let mut b = ValveBitMask(0b111);
        b.remove(1);
        assert_eq!(b.0, 0b101);
    }

    #[test]
    fn iterate() {
        let mut i = ValveBitMask(0b101).iter();
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), None);
        let mut i = ValveBitMask(0b1100).iter();
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), None);
    }
}
