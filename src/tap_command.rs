use nom::sequence::{preceded, tuple};
use nom::{branch::alt, combinator::value, bytes::complete::tag, sequence::terminated, IResult};
use nom::character::complete::space0;

use crate::string_parser::string;
use crate::{is_last, parse_bool, parse_list2, parse_object2, parse_spacer};

#[derive(Debug, Clone)]
pub struct TapCommand<'a> {
    pkg: &'a str,
    url: &'a str,
}

impl<'a> TapCommand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        // Allocate the structure data
        let mut brew = Self {
            pkg: "",
            url: "",
        };

        // Get the initial command
        let (remainder, pkg) = string::<()>(input).unwrap();
        brew.pkg = pkg;

        let (remainder, last) = is_last(remainder)?;
        if last {
            return Ok((remainder, brew));
        }

        // Get the URL string
        let (remainder, url) = string(remainder)?;
        brew.url = url;

        Ok((remainder, brew))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let (remainder, brew) = TapCommand::parse("\"package\" \n").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(remainder, "");

        // Remainder must be after the line break
        let (remainder, brew) = TapCommand::parse("\"package\" \nextra").unwrap();
        assert_eq!(brew.pkg, "package");
        assert_eq!(remainder, "extra");

        // Returns an error when following invalid input
        let res = TapCommand::parse("\"pkg\", \nextra");
        match res {
            Err(nom::Err::Error(err)) => {
                assert_eq!(err.to_string(), "error Char at: \nextra"); // Checking the remaining input
            }
            _ => panic!("Expected an error but got: {:?}", res),
        }
    }

    #[test]
    fn parse_url() {
        let (remainder, brew) = TapCommand::parse("\"package\", \"url\"\n").unwrap();
        assert_eq!(brew.url, "url");
        assert_eq!(remainder, "\n");

        // Returns an error when following invalid input
        let res = TapCommand::parse("\"package\", invalid\nextra");
        match res {
            Err(nom::Err::Error(err)) => {
                assert_eq!(err.to_string(), "error Char at: invalid\nextra"); // Checking the remaining input
            }
            _ => panic!("Expected an error but got: {:?}", res),
        }
    }
}

