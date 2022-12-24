use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    multi::separated_list0,
    sequence::tuple,
};

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct ValveLabel([u8; 2]);

#[derive(Debug, PartialEq)]
pub struct Valve {
    pub flow_rate: u32,
    pub tunnels: Vec<ValveLabel>,
}

#[derive(Debug)]
pub struct Cave(pub HashMap<ValveLabel, Valve>);

impl Display for ValveLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap_or("??"))
    }
}

impl std::fmt::Debug for ValveLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl FromStr for ValveLabel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 || !s.chars().all(|c| c.is_ascii()) {
            return Err(anyhow!("Invalid valve label '{s}'"));
        }
        Ok({
            let iter = s.bytes();
            let mut iter = iter.into_iter();
            ValveLabel {
                0: [
                    iter.next().unwrap_or('?' as u8),
                    iter.next().unwrap_or('?' as u8),
                ],
            }
        })
    }
}

impl std::ops::Deref for Cave {
    type Target = HashMap<ValveLabel, Valve>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_valve(i: &str) -> nom::IResult<&str, (ValveLabel, Valve)> {
    let (i, (_, label, _, flow_rate, _, _, tunnels)) = tuple((
        tag("Valve "),
        take(2usize),
        tag(" has flow rate="),
        digit1,
        alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
        alt((tag("valves "), tag("valve "))),
        separated_list0(tag(", "), take(2usize)),
    ))(i)?;
    Ok((
        i,
        (
            label.parse().map_err(|_| {
                nom::Err::Error(nom::error::Error {
                    input: i,
                    code: nom::error::ErrorKind::Tag,
                })
            })?,
            Valve {
                flow_rate: flow_rate.parse().map_err(|_| {
                    nom::Err::Error(nom::error::Error {
                        input: i,
                        code: nom::error::ErrorKind::Digit,
                    })
                })?,
                tunnels: tunnels
                    .iter()
                    .map(|e| e.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| {
                        nom::Err::Error(nom::error::Error {
                            input: i,
                            code: nom::error::ErrorKind::Tag,
                        })
                    })?,
            },
        ),
    ))
}

#[test]
fn test_parse_valve() {
    let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
    assert_eq!(
        parse_valve(s).unwrap().1,
        (
            "AA".parse().unwrap(),
            Valve {
                flow_rate: 0,
                tunnels: vec![
                    "DD".parse().unwrap(),
                    "II".parse().unwrap(),
                    "BB".parse().unwrap()
                ]
            }
        )
    );
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cave(
            s.lines()
                .map(|ln| parse_valve(ln).map(|(_, v)| v))
                .collect::<Result<HashMap<ValveLabel, Valve>, _>>()
                .map_err(|e| anyhow::anyhow!("Error parsing input: {e}"))?,
        ))
    }
}
