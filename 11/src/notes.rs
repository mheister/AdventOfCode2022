use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, one_of, space0},
    combinator::{map_res, opt},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Monkey {
    pub idx: usize,
    pub starting_items: Vec<usize>,
    pub operation: Operation,
    pub test: Test,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Old,
    Constant(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Test {
    pub divisor: usize,
    pub true_target: usize,
    pub false_target: usize,
}

fn parse_monkey_head(input: &str) -> IResult<&str, usize> {
    let monkey_marker = tag("Monkey");
    let idx = map_res(digit1, str::parse);
    let (input, (_, _, idx, _)) = tuple((monkey_marker, space0, idx, tag(":")))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, idx))
}

#[test]
fn test_parse_monkey_head() {
    let input = "Monkey 11:\n";
    let (rest, result) = parse_monkey_head(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(result, 11);
}

fn parse_monkey_starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    let mut result = vec![];
    let (input, _) = tuple((tag("  Starting items:"), space0))(input)?;
    let mut input = input;
    let mut item: Option<usize>;
    loop {
        (input, _) = space0(input)?;
        (input, item) = opt(map_res(digit1, str::parse))(input)?;
        if item.is_none() {
            break;
        }
        result.push(item.unwrap());
        (input, _) = opt(tag(","))(input)?;
    }
    let (input, _) = newline(input)?;
    Ok((input, result))
}

#[test]
fn test_parse_monkey_starting_items() {
    let input = "  Starting items: 79, 98\n";
    let (rest, result) = parse_monkey_starting_items(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(result, vec![79, 98]);
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    let (input, maybe_old) = opt(tag("old"))(input)?;
    if maybe_old.is_some() {
        return Ok((input, Operand::Old));
    }
    let (input, constant) = map_res(digit1, str::parse)(input)?;
    return Ok((input, Operand::Constant(constant)));
}

fn parse_monkey_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tuple((
        tag("  Operation:"),
        space0,
        tag("new"),
        space0,
        tag("="),
        space0,
    ))(input)?;
    let (input, (operand1, _, operation, _, operand2)) =
        tuple((parse_operand, space0, one_of("+*"), space0, parse_operand))(input)?;
    let (input, _) = newline(input)?;
    match operation {
        '+' => Ok((input, Operation::Add(operand1, operand2))),
        '*' => Ok((input, Operation::Multiply(operand1, operand2))),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

#[test]
fn test_parse_monkey_operation() {
    let input = "  Operation: new = old * 19\n";
    let (rest, result) = parse_monkey_operation(input).unwrap();
    assert_eq!(
        result,
        Operation::Multiply(Operand::Old, Operand::Constant(19))
    );
    assert_eq!(rest, "");
}

fn parse_monkey_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = tuple((tag("  Test: divisible by"), space0))(input)?;
    let (input, divisor) = map_res(digit1, str::parse)(input)?;
    let (input, (_, _, _, true_target, _)) = tuple((
        newline,
        tag("    If true: throw to monkey"),
        space0,
        map_res(digit1, str::parse),
        space0,
    ))(input)?;
    let (input, (_, _, _, false_target, _)) = tuple((
        newline,
        tag("    If false: throw to monkey"),
        space0,
        map_res(digit1, str::parse),
        space0,
    ))(input)?;
    Ok((
        input,
        Test {
            divisor,
            true_target,
            false_target,
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (idx, starting_items, operation, test)) = tuple((
        parse_monkey_head,
        parse_monkey_starting_items,
        parse_monkey_operation,
        parse_monkey_test,
    ))(input)?;
    Ok((
        input,
        Monkey {
            idx,
            starting_items,
            operation,
            test,
        },
    ))
}

fn parse_notes_internal(input: &str) -> IResult<&str, Vec<Monkey>> {
    let mut result = vec![];
    let mut input = input;
    loop {
        (input, _) = space0(input)?;
        let monkey;
        (input, monkey) = opt(parse_monkey)(input)?;
        if monkey.is_none() {
            break;
        }
        result.push(monkey.unwrap());
        (input, _) = opt(multispace0)(input)?;
    }
    Ok((input, result))
}

pub fn parse_notes(input: &str) -> Result<Vec<Monkey>, String> {
    let res = parse_notes_internal(input);
    res.map_err(|err| err.to_string())
        .map(|(_, monkeys)| monkeys)
}
