use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SectionRange {
    start: u32,
    end: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ElvePair(SectionRange, SectionRange);

impl SectionRange {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    #[must_use]
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    #[must_use]
    fn overlaps_with(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut limits = s.split('-');
        let errmsg = "Expected range in fomrat start-end, e.g., 7-15";
        let (start, end) = (limits.next().ok_or(errmsg)?, limits.next().ok_or(errmsg)?);
        let (start, end) = (
            u32::from_str(start).map_err(|_| "Error parsing start index")?,
            u32::from_str(end).map_err(|_| "Error parsing end index")?,
        );
        if limits.next().is_some() {
            return Err(errmsg.to_owned());
        }
        Ok(SectionRange { start, end })
    }
}

impl ElvePair {
    #[must_use]
    fn one_section_range_contains_the_other(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    #[must_use]
    fn section_ranges_overlap(&self) -> bool {
        self.0.overlaps_with(&self.1)
    }
}

impl FromStr for ElvePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s.split(',');
        let errmsg = "Expected exactly two ranges separated by comma";
        let (r0, r1) = (ranges.next().ok_or(errmsg)?, ranges.next().ok_or(errmsg)?);
        if ranges.next().is_some() {
            return Err(errmsg.to_owned());
        }
        Ok(ElvePair(
            SectionRange::from_str(r0)?,
            SectionRange::from_str(r1)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_range_shoud_correctly_report_whether_it_fully_contains_another() {
        let tests = vec![
            (SectionRange::new(1, 2), SectionRange::new(2, 3), false),
            (SectionRange::new(1, 4), SectionRange::new(2, 5), false),
            (SectionRange::new(1, 4), SectionRange::new(2, 3), true),
            (SectionRange::new(2, 3), SectionRange::new(1, 3), false),
            (SectionRange::new(1, 2), SectionRange::new(1, 2), true),
        ];
        for (range1, range2, fully_contained) in tests {
            assert_eq!(range1.fully_contains(&range2), fully_contained);
        }
    }

    #[test]
    fn elve_pairs_should_correctly_determine_whether_one_section_range_is_fully_contained_in_the_other(
    ) {
        let test_data = vec![
            ("2-4,6-8", false),
            ("2-3,4-5", false),
            ("5-7,7-9", false),
            ("2-8,3-7", true),
            ("6-6,4-6", true),
            ("2-6,4-8", false),
        ];
        for (pair_str, fully_contained) in test_data {
            let pair = ElvePair::from_str(pair_str).unwrap();
            assert_eq!(
                pair.one_section_range_contains_the_other(),
                fully_contained,
                "{}, parsed as {:?}",
                pair_str,
                pair
            );
        }
    }

    #[test]
    fn elve_pairs_should_correctly_determine_overlap(
    ) {
        let test_data = vec![
            ("2-4,6-8", false),
            ("2-3,4-5", false),
            ("5-7,7-9", true),
            ("2-8,3-7", true),
            ("6-6,4-6", true),
            ("2-6,4-8", true),
        ];
        for (pair_str, overlap) in test_data {
            let pair = ElvePair::from_str(pair_str).unwrap();
            assert_eq!(
                pair.section_ranges_overlap(),
                overlap,
                "{}, parsed as {:?}",
                pair_str,
                pair
            );
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let count_fully_contained = BufReader::new(file)
        .lines()
        .map(|ln| ElvePair::from_str(&ln.unwrap()).unwrap())
        .map(|pair| ElvePair::one_section_range_contains_the_other(&pair))
        .filter(|x| *x)
        .count();
    println!(
        "Number of elve pairs where one assigned section range contains the other: {}",
        count_fully_contained
    );
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let count_overlap = BufReader::new(file)
        .lines()
        .map(|ln| ElvePair::from_str(&ln.unwrap()).unwrap())
        .map(|pair| ElvePair::section_ranges_overlap(&pair))
        .filter(|x| *x)
        .count();
    println!(
        "Number of elve pairs where the assigned section ranges overlap: {}",
        count_overlap
    );
}
