use std::str::FromStr;

// use brewfile_traits::Deserialize;
use nom::branch::alt;
use nom::bytes::{is_not, take_till, take_until};
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::{AsChar, IResult};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0, space0, space1};
use nom::bytes::complete::tag;
use nom::Parser;

pub fn generate_commands(input: &str) -> IResult<&str, Vec<Cmd>> {
    let mut cmds: Vec<Cmd> = Vec::new();
    let mut input = input;

    loop {
        let (remainder, _) = multispace0(input)?;
        if remainder.is_empty() {
            break;
        }

        let (remainder, value) = get_command(remainder).unwrap();
        cmds.push(value);

        // Update the input for the next loop
        input = remainder;
    }

    Ok((input, cmds))
}

pub fn get_command(input: &str) -> IResult<&str, Cmd> {
    let mut ident: Vec<Ident> = Vec::new();

    let (input, _) = multispace0(input)?;

    let (input, program) = program(input).unwrap();

    // Clear the first space
    let (remainder, _) = parse_spaces(input).unwrap();

    let mut input = remainder;
    loop {
        if input.is_empty() {
            break;
        }

        let (remainder, value) = get_arg(input).unwrap();
        ident.push(value);

        let (remainder, flag) = has_options(remainder);

        // Update the input for the next loop.
        input = remainder;
        if !flag {
            break;
        }
    }

    let cmd = Cmd { cmd: String::from(program), args: ident };
    Ok((input, cmd))
}

pub fn parse_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?; // Propagate the error
    Ok((input, ()))
}

pub fn parse_string(input: &str) -> IResult<&str, String> {
    let (remainder, value) = alt((
        delimited(char('"'), is_not("\""), char('"')),
    )).parse(input)?;
    Ok((remainder, String::from(value)))
}

pub fn parse_pair(input: &str) -> IResult<&str, (String, String)> {
    let (remainder, (key, value)) = separated_pair(parse_string, kvp_sep, parse_string).parse(input)?;
    let tupple = (key, value);
    Ok((remainder, tupple))
}

pub fn kvp_sep(input: &str) -> IResult<&str, ()> {
    let (remainder, _) = terminated(tag(":"), space0).parse(input)?;
    Ok((remainder, ()))
}

pub fn parse_kvp_list(input: &str) -> IResult<&str, Vec<(String, String)>> {
    let (remainder, hash) = separated_list0(separator, parse_pair).parse(input)?;
    Ok((remainder, hash))
}

pub fn separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = preceded(space0, terminated(tag(","), space0)).parse(input)?;
    Ok((input, ()))
}

pub fn program(input: &str) -> IResult<&str, &str> {
    let (remainder,  value) = alpha1(input)?;
    Ok((remainder, value))
}

pub fn string_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, value) = parse_string(input)?;
    let value = Ident::Str(value);
    Ok((remainder, value))
}

pub fn named_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, arg_name) = take_till(|c| c == ':').parse(input)?;
    let (remainder, _) = tag(":").parse(remainder)?;

    // Make sure the spaces are consumed
    let (remainder, _) = space1(remainder)?;

    let (remainder, arg_value) = alt((
        symbol_value,
        list_value,
        pair_value,
        boolean_value,

    )).parse(remainder)?;
    let value = Ident::Named(String::from(arg_name), Box::new(arg_value));
    Ok((remainder, value))
}

pub fn list_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, values) = delimited(
        terminated(char('['), space0),
        separated_list0(separator, parse_string),
        preceded(space0, char(']'))
    ).parse(input)?;

    let value = Ident::List(values);
    Ok((remainder, value))
}

pub fn pair_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, values) = delimited(
        terminated(char('{'), space0),
        parse_kvp_list,
        preceded(space0, char('}'))
    ).parse(input)?;

    let value = Ident::Map(values);
    Ok((remainder, value))
}

pub fn symbol_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, value) = preceded(tag(":"), alphanumeric1).parse(input)?;
    Ok((remainder, Ident::Symbol(String::from(value))))
}

pub fn boolean_value(input: &str) -> IResult<&str, Ident> {
    let (remainder, result) = alt((
        value(true, tag("true")),
        value(false, tag("false")),
    )).parse(input)?;

    Ok((remainder, Ident::Bool(result)))
}

pub fn has_options(input: &str) -> (&str, bool) {
    // peek(separator).parse(input).is_ok()
    match separator.parse(input) {
        Ok((input, _)) => (input, true),
        Err(_) => (input, false)
    }
}

pub fn get_arg(input: &str) -> IResult<&str, Ident> {
    let (remainder, value) = alt((
        string_value,
        named_value,
    )).parse(input)?;

    Ok((remainder, value))
}

#[derive(Debug)]
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<Ident>
}

#[derive(Debug)]
pub enum Ident {
    Str(String),
    List(Vec<String>),
    Map(Vec<(String, String)>),
    Pair(String, String),
    Bool(bool),
    Named(String, Box<Ident>),
    Symbol(String),
}

pub trait ParseIdent<T> {
    fn parse(&self) -> Result<T, String>;
}
impl<T> ParseIdent<T> for Ident
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn parse(&self) -> Result<T, String> {
        match self {
            Ident::Str(s) => s.parse::<T>().map_err(|e| format!("Failed to parse '{}' as target type: {}", s, e)),
            _ => Err(format!("Cannot parse Ident variant into target type {:#?}", self)),
        }
    }
}


/**
 * META
 **/
pub fn parse_meta(input: &str) -> IResult<&str, Vec<Ident>> {
    // Consume the begining of the meta comment
    let (input, _) = multispace0(input)?;
    let (remainder, input) = delimited(tag("/**"), take_until("*/"), tag("*/")).parse(input)?;

    let mut flags: Vec<Ident> = Vec::new();

    // Parse the comment start
    let mut input = input;
    loop {
        let (input1, _) = multispace0(input)?;
        if input1.is_empty() {
            break;
        }

        let (input1, value) = parse_meta_comment(input1)?;
        input = input1;
        flags.push(value);

    }

    Ok((remainder, flags))
}

pub fn parse_meta_comment(input: &str) -> IResult<&str, Ident> {
    let (input, _) = preceded(multispace0, tag("*")).parse(input)?;
    let (input, flag_name) = preceded(space0, preceded(tag("@"), alphanumeric1)).parse(input)?;
    let (input, flag_value) = preceded(space0, take_till(AsChar::is_newline)).parse(input)?;

    let named = Ident::Pair(String::from(flag_name), String::from(flag_value));
    Ok((input, named))
}

