use nom::branch::alt;
use nom::character::complete::{multispace0, space0};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{IResult, Parser};
use string_parser::string;

use crate::string_parser;

pub fn is_last(input: &str) -> IResult<&str, bool> {
    let (remainder, comma) = alt((parse_spacer, multispace0)).parse(input)?;
    match comma {
        "," => Ok((remainder, false)),
        _ => Ok((remainder, true)),
    }
}

pub fn parse_spacer(input: &str) -> IResult<&str, &str> {
    preceded(space0, terminated(tag(","), space0)).parse(input)
}

pub fn parse_list(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        terminated(tag("["), space0),
        separated_list0(parse_spacer, string),
        preceded(space0, tag("]")),
    ).parse(input)
}

pub fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        string,
        terminated(tag(":"), space0),
        string
    ).parse(input)
}

pub fn parse_object(input: &str) -> IResult<&str, Vec<&str>> {
    let (remainder, pairs) = delimited(
        terminated(tag("{"), space0),
        separated_list0(parse_spacer, key_value),
        preceded(space0, tag("}")),
    ).parse(input)?;

    let flat = Vec::with_capacity(pairs.len() * 2);
    let flat = pairs.iter()
        .fold(flat, |mut acc, p| { acc.extend(&[p.0, p.1]); acc });

    Ok((remainder, flat))
}

// A custom parser that takes another parser as a parameter
// The parser ensures that the input is surrounded by optional whitespace.
// #[allow(dead_code)]
// pub fn parse_line<'a, O, F>(parser: F) -> impl Parser(&'a str) -> IResult<&'a str, O>
// where F: FnMut(&'a str) -> IResult<&'a str, O>,
// {
//     delimited(multispace0, parser, multispace0)
// }

// #[cfg(test)]
// mod tests {
//     use nom::character::complete::{alpha1, digit1};
//
//     #[test]
//     fn parse_line_single() {
//         let (remainder, value) = parse_line(alpha1)("package").unwrap();
//         assert_eq!(value, "package");
//         assert_eq!(remainder, "");
//
//         // Leading white spaces
//         let (remainder, value) = parse_line(alpha1)("\npackage").unwrap();
//         assert_eq!(value, "package");
//         assert_eq!(remainder, "");
//
//         // Trailing white spaces
//         let (remainder, value) = parse_line(alpha1)("package\n").unwrap();
//         assert_eq!(value, "package");
//         assert_eq!(remainder, "");
//
//         // Multiple lines
//         let (remainder, value) = parse_line(digit1)("\n\n\n12\n\nother\n\n\n").unwrap();
//         assert_eq!(value, "12");
//         assert_eq!(remainder, "other\n\n\n");
//     }
// }

