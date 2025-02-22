use nom::Parser;
use nom::{branch::alt, combinator::value, bytes::complete::tag, sequence::terminated, IResult};
use nom::character::complete::space0;

use brewfile_parser::string_parser::string;
use brewfile_parser::parsers::{is_last, parse_list, parse_object};

#[derive(Debug, Clone)]
pub struct BrewCommand<'a> {
    pkg: String,
    pub args: Vec<&'a str>,
    pub link: LinkOptions,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkOptions {
    None,
    On,
    Override,
}

impl<'a> BrewCommand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        // Allocate the structure data
        let mut brew = Self {
            pkg: String::new(),
            args: Vec::new(),
            link: LinkOptions::None,
        };

        // Get the initial command
        let (remainder, pkg) = string::<()>(input).unwrap();
        brew.pkg= pkg.to_string();

        // Check if this is the last parameter on the set
        let (remainder, last) = is_last(remainder).unwrap();
        let mut last = last;
        let mut result_remainder = remainder;

        // Loop over all the parameters and update as needed
        while !last {
            let (remainder, key) = terminated(alt((tag("args"), tag("link"))), terminated(tag(":"), space0)).parse(result_remainder)?;
            let remainder = match key {
                "args" => {
                    let (remainder, value) = alt((parse_list, parse_object)).parse(remainder)?;
                    brew.args = value;
                    remainder
                },
                "link" => {
                    let (remainder, value) = alt((
                        value(LinkOptions::On, tag("true")),
                        value(LinkOptions::Override, tag(":override"))
                    )).parse(remainder)?;
                    brew.link = value;
                    remainder
                },
                unknown => panic!("Unknown parameter {unknown}"),
            };

            let (remainder, check) = is_last(remainder)?;
            last = check;
            result_remainder  = remainder;
        };

        Ok((result_remainder, brew))
    }

    pub fn pkg(self) -> String {
        self.pkg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let (remainder, brew) = BrewCommand::parse("\"package\" \n").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(remainder, "");

        // Remainder must be after the line break
        let (remainder, brew) = BrewCommand::parse("\"package\" \nextra").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(remainder, "extra");

        // Returns an error when following invalid input
        let res = BrewCommand::parse("\"package\", invalid: true\nextra");
        match res {
            Err(nom::Err::Error(err)) => {
                assert_eq!(err.to_string(), "error Tag at: invalid: true\nextra"); // Checking the remaining input
            }
            _ => panic!("Expected an error but got: {:?}", res),
        }
    }

    #[test]
    fn parse_args() {
        let (remainder, brew) = BrewCommand::parse("\"pkg\", args: [\"hello\", \"world\"]\n").unwrap();
        assert_eq!(brew.args, vec!["hello", "world"]);
        assert_eq!(remainder, "");

        let (remainder, brew) = BrewCommand::parse("\"pkg\", args: {\"hello\": \"world\"}\n").unwrap();
        assert_eq!(brew.args, vec!["hello", "world"]);
        assert_eq!(remainder, "");

        // Should only accept maps or lists
        let res = BrewCommand::parse("\"pkg\", args: hello\n");
        match res {
            Err(nom::Err::Error(err)) => {
                assert_eq!(err.to_string(), "error Tag at: hello\n"); // Checking the remaining input
            }
            _ => panic!("Expected an error but got: {:?}", res),
        }

        // Should fail on dangling args
        let res = BrewCommand::parse("\"pkg\", args: \n");
        match res {
            Err(nom::Err::Error(err)) => {
                assert_eq!(err.to_string(), "error Tag at: \n"); // Checking the remaining input
            }
            _ => panic!("Expected an error but got: {:?}", res),
        }
    }

    #[test]
    fn parse_link() {
        let (remainder, brew) = BrewCommand::parse("\"package\", link: true \n").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(brew.link, LinkOptions::On);
        assert_eq!(remainder, "");

        let (remainder, brew) = BrewCommand::parse("\"package\", link: :override").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(brew.link, LinkOptions::Override);
        assert_eq!(remainder, "");
    }
}

