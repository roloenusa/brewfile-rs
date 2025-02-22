use nom::branch::alt;
use nom::sequence::terminated;
use nom::Parser;
use nom::{bytes::complete::tag, IResult};
use nom::character::complete::{alpha1, space1};

use crate::brew_command;
use crate::metafield::Metafield;
use crate::tap_command;

#[derive(Debug)]
pub enum Command<'a> {
    Brew(brew_command::BrewCommand<'a>),
    Tap(tap_command::TapCommand<'a>),
    None,
}

#[derive(Debug)]
pub struct MetaCommand<'a> {
    description: &'a str,
    optional: bool,
    pub command: Command<'a>,
}

fn parse(input: &str) -> IResult<&str, MetaCommand> {

    let mut metacommand: MetaCommand = MetaCommand {
        description: "",
        optional: false,
        command: Command::None,
    };

    let mut res_remainder = input;
    let mut sentinel = true;
    while !res_remainder.is_empty() && sentinel {
        let (remainder, command) = alt((
            terminated(alpha1, space1),
            terminated(tag("##"), space1),
        )).parse(res_remainder.trim())?;

        res_remainder = match command {
            "tap" => {
                let (remainder, tap) = tap_command::TapCommand::parse(remainder)?;
                metacommand.command = Command::Tap(tap);
                sentinel = false;
                remainder
            }
            "brew" => {
                let (remainder, brew) = brew_command::BrewCommand::parse(remainder)?;
                metacommand.command = Command::Brew(brew);
                sentinel = false;
                remainder
            }
            "##" => {
                let (remainder, metadata) = Metafield::parse(remainder)?;
                match metadata {
                    Metafield::Optional => metacommand.optional = true,
                    Metafield::Description(value) => metacommand.description = value,
                };
                remainder
            }
            unknown => panic!("Unknown command: {unknown} - {remainder}"),
        };
    };

    Ok((res_remainder, metacommand))
}

pub fn parse_command(input: &str) -> IResult<&str, Vec<MetaCommand>> {
    let mut input = input.trim();

    let mut commands: Vec<MetaCommand> = Vec::new();
    while !input.is_empty() {
        let (remainder, metacommand) = parse(input)?;
        input = remainder;
        commands.push(metacommand);
    };
    Ok((input, commands))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_metadata() {
        let (remainder, brew) = parse_command("brew \"package\" \n").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(brew.len(), 1);
        assert_eq!(brew[0].description, "");
        assert!(!brew[0].optional);
        match &brew[0].command {
            Command::Brew(a) => {
                assert_eq!(a.clone().pkg(), "package");
            },
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn parse_metadata() {
        let text = "\
            ## @description This is a description
            ## @optional
            brew \"package\" \n
        ";

        let (remainder, brew) = parse_command(text).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(brew.len(), 1);
        assert_eq!(brew[0].description, "This is a description");
        assert!(brew[0].optional);
        match &brew[0].command {
            Command::Brew(a) => {
                assert_eq!(a.clone().pkg(), "package");
            },
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn parse_multi_metadata() {
        let text = "\
            ## @description This is a description
            ## @optional
            brew \"package\" \n

            ## @description Other description
            brew \"package2\" \n
        ";

        let (remainder, brew) = parse_command(text).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(brew.len(), 2);
        assert_eq!(brew[0].description, "This is a description");
        assert!(brew[0].optional);
        match &brew[0].command {
            Command::Brew(a) => {
                assert_eq!(a.clone().pkg(), "package");
            },
            _ => panic!("wrong command"),
        }

        assert_eq!(brew[1].description, "Other description");
        assert!(!brew[1].optional);
        match &brew[1].command {
            Command::Brew(a) => {
                assert_eq!(a.clone().pkg(), "package2");
            },
            _ => panic!("wrong command"),
        }
    }
}

