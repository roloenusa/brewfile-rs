use nom::{branch::alt, combinator::value, bytes::complete::tag, sequence::terminated, IResult};
use nom::character::complete::space0;

use crate::string_parser::string;
use crate::parsers::{is_last, parse_list, parse_object};

pub struct Command {
    prg: String,
    pkg: String,
    args: Vec<Arguments>,
}

pub struct Arguments {
    option: String,
    args: ArgumentValue,
}

pub enum ArgumentValue {
    String(String),
    List(Vec<String>),
}

fn parse_cmd(input: &str) -> IResult<&str, Command> {
    let command = Command {
        prg: String::new(),
        pkg: String::new(),
        args: Vec::new(),
    };

    // Get the initial command
    let (remainder, pkg) = string::<()>(input).unwrap();
    command.pkg= pkg.to_string();

    // Check if this is the last parameter on the set
    let (remainder, last) = is_last(remainder)?;
    let mut last = last;
    let mut result_remainder = remainder;

    // Loop over all the parameters and update as needed
    while !last {
        let (remainder, key) = terminated(alt((tag("args"), tag("link"))), terminated(tag(":"), space0))(result_remainder)?;
        let remainder = match key {
            "args" => {
                let (remainder, value) = alt((parse_list, parse_object))(remainder)?;
                command.args = value;
                remainder
            },
            "link" => {
                let (remainder, value) = alt((
                    value(LinkOptions::On, tag("true")),
                    value(LinkOptions::Override, tag(":override"))
                ))(remainder)?;
                command.link = value;
                remainder
            },
            unknown => panic!("Unknown parameter {unknown}"),
        };

        let (remainder, check) = is_last(remainder)?;
        last = check;
        result_remainder  = remainder;
    };

    Ok((result_remainder, command))
}

