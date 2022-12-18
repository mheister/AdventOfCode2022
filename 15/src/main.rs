use common::twod::Point;
use nom::{bytes::complete::tag, sequence::tuple};
use std::{env, fs, collections::HashSet};

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
}

fn main() -> anyhow::Result<()> {
    let input_file_path = env::args().nth(1).unwrap_or("15/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let row_of_interest: i32 = env::args()
        .nth(2)
        .and_then(|a| a.parse::<i32>().ok())
        .unwrap_or(2000000);
    let mut no_beacon_coverage = RowCoverage::new();
    let mut beacons_in_line = HashSet::<i32>::new();
    for ln in input.lines() {
        let (_, rep) = parse_sensor_report(ln).expect("error parsing ln");
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
    let covered = no_beacon_coverage.covered_len() as usize - beacons_in_line.len();
    println!("Number of covered postitions: {covered}");
    Ok(())
}
