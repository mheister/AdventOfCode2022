use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    multi::separated_list0,
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
pub struct Cave(HashMap<String, Valve>);

fn parse_valve(i: &str) -> nom::IResult<&str, (String, Valve)> {
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
            label.to_string(),
            Valve {
                flow_rate: flow_rate.parse().map_err(|_| {
                    nom::Err::Error(nom::error::Error {
                        input: i,
                        code: nom::error::ErrorKind::Digit,
                    })
                })?,
                tunnels: tunnels.iter().map(|e| e.to_string()).collect(),
            },
        ),
    ))
}

#[test]
fn test_parse_valve() {
    let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
    assert_eq!(
        parse_valve(s).unwrap().1 .1,
        Valve {
            flow_rate: 0,
            tunnels: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()]
        }
    );
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cave(
            s.lines()
                .map(|ln| parse_valve(ln).map(|(_, v)| v))
                .collect::<Result<HashMap<String, Valve>, _>>()
                .map_err(|e| anyhow::anyhow!("Error parsing input: {e}"))?,
        ))
    }
}
