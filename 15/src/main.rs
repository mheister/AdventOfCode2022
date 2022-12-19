use common::twod::Point;
use nom::{bytes::complete::tag, sequence::tuple};
use std::{collections::HashSet, env, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SensorReport {
    sensor: Point,
    beacon: Point,
}

fn parse_sensor_report(input: &str) -> nom::IResult<&str, SensorReport> {
    let (input, (_, sensor_x, _, sensor_y, _, beacon_x, _, beacon_y)) = tuple((
        tag("Sensor at x="),
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
        tag(": closest beacon is at x="),
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
    ))(input)?;
    Ok((
        input,
        SensorReport {
            sensor: Point {
                x: sensor_x,
                y: sensor_y,
            },
            beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
        },
    ))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, x: i32) -> bool {
        self.start <= x && x <= self.end
    }
    fn overlaps(&self, other: Range) -> bool {
        self.contains(other.start) || other.contains(self.start)
    }
}

#[derive(Debug)]
struct RowCoverage {
    ranges: Vec<Range>,
}

impl RowCoverage {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn cover(&mut self, mut range: Range) {
        self.ranges = self
            .ranges
            .iter()
            .cloned()
            .filter(|r| {
                if r.overlaps(range) {
                    range.start = std::cmp::min(r.start, range.start);
                    range.end = std::cmp::max(r.end, range.end);
                    false
                } else {
                    true
                }
            })
            .collect();
        self.ranges.push(range);
        self.ranges.sort();
    }

    fn covered_len(&self) -> i32 {
        self.ranges.iter().map(|r| r.end - r.start + 1).sum()
    }

    fn narrow(&self, range: Range) -> RowCoverage {
        let mut iter = self.ranges.iter();
        let mut narrowed_ranges = vec![];
        if let Some(mut first_relevant) = iter.find(|r| r.end >= range.start).cloned() {
            first_relevant.start = std::cmp::max(first_relevant.start, range.start);
            narrowed_ranges.push(first_relevant);
            for r in iter {
                if r.start > range.end {
                    break;
                }
                narrowed_ranges.push(r.clone());
            }
            let last_relevant = narrowed_ranges.last_mut().unwrap();
            last_relevant.end = std::cmp::min(last_relevant.end, range.end);
        }
        RowCoverage {
            ranges: narrowed_ranges,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input_file_path = env::args().nth(1).unwrap_or("15/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let input_kind = env::args().nth(2).unwrap_or("full".to_owned());
    let (row_of_interest, xy_max) = match input_kind.as_str() {
        "test" => (10, 20),
        _ => (2000000, 4000000),
    };
    let reports: Vec<_> = input
        .lines()
        .map(|ln| {
            let (_, rep) = parse_sensor_report(ln).expect("error parsing ln");
            rep
        })
        .collect();
    let covered = part1_get_num_covered_positions(&reports, row_of_interest);
    println!("Number of covered postitions: {covered}");
    let distress_beacon_pos =
        part2_get_beacon_position(&reports, xy_max).expect("Distres beacon was not found");
    let tuning_frequency =
        4000000 * distress_beacon_pos.0 as usize + distress_beacon_pos.1 as usize;
    println!("Tuning frequency is {tuning_frequency}");
    Ok(())
}

fn part1_get_num_covered_positions(
    reports: &Vec<SensorReport>,
    row_of_interest: i32,
) -> usize {
    let mut no_beacon_coverage = RowCoverage::new();
    let mut beacons_in_line = HashSet::<i32>::new();
    for rep in reports {
        if rep.beacon.y == row_of_interest {
            beacons_in_line.insert(rep.beacon.x);
        }
        let dist =
            (rep.sensor.x - rep.beacon.x).abs() + (rep.sensor.y - rep.beacon.y).abs();
        let reach = dist - (row_of_interest - rep.sensor.y).abs();
        if reach > 0 {
            no_beacon_coverage.cover(Range {
                start: rep.sensor.x - reach,
                end: rep.sensor.x + reach,
            });
        }
    }
    no_beacon_coverage.covered_len() as usize - beacons_in_line.len()
}

fn part2_get_beacon_position(
    reports: &Vec<SensorReport>,
    xy_max: i32,
) -> Option<(i32, i32)> {
    for y in 0..xy_max {
        let mut no_beacon_coverage = RowCoverage::new();
        for rep in reports {
            let dist =
                (rep.sensor.x - rep.beacon.x).abs() + (rep.sensor.y - rep.beacon.y).abs();
            let reach = dist - (y - rep.sensor.y).abs();
            if reach > 0 {
                no_beacon_coverage.cover(Range {
                    start: rep.sensor.x - reach,
                    end: rep.sensor.x + reach,
                });
            }
        }
        let no_beacon_coverage = no_beacon_coverage.narrow(Range {
            start: 0,
            end: xy_max,
        });
        if no_beacon_coverage.covered_len() < xy_max + 1 {
            let x = no_beacon_coverage
                .ranges
                .first()
                .map(|r| r.end + 1)
                .unwrap_or(0);
            return Some((x, y));
        }
    }
    None
}
