// mod brew_command;
// mod tap_command;
// mod metadata;
// mod metafield;

// use metadata::{parse_command, MetaCommand};
// use nom::IResult;

// fn parse_input(input: &str) -> IResult<&str, Vec<MetaCommand>> {
//     parse_command(input)
// }
//
// fn main() {
//     let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
//
//     // println!("{}", src);
//
//     let (_remainder, result) = parse_input(&src).unwrap();
//
//     // println!("remainder: {:#?}", remainder);
//
//     // for command in &result {
//     //     println!("{:#?}", command);
//     // }
//     let metacommand = &result.last().unwrap();
//     match &metacommand.command {
//         metadata::Command::Tap(value) => value.install(),
//         _ => panic!("--- panic"),
//     };
// }


use std::collections::HashMap;

use brewfile_derive::CmdParser;

#[derive(CmdParser, Debug)]
struct BrewMacro {
    first: String,

    #[arg]
    second: String,

    third: u32,
}

// #[derive(Debug)]
// struct P {}

fn main() {

    println!("Hello world");

    let bm = BrewMacro::init(String::from("Hello world"));
    println!("{:#?}", bm);


    let bm = BrewMacro::parse("tap \"world\"");
    println!("{:#?}", bm);

    let mut data = HashMap::new();
    data.insert("first".to_string(), "42".to_string());
    data.insert("second".to_string(), "Alice".to_string());
    data.insert("third".to_string(), "30".to_string());
    match BrewMacro::parse_t(data) {
        Ok(user) => println!("Parsed user: {:?}", user),
        Err(err) => eprintln!("Failed to parse: {}", err),
    };

    let a = String::from("hello");
    let b = a.clone();

    let mut data: Vec<brewfile_parser::ast::Ident> = Vec::new();
    data.push(brewfile_parser::ast::Ident::Str(String::from("hello")));
    data.push(brewfile_parser::ast::Ident::Str(String::from("world")));
    data.push(brewfile_parser::ast::Ident::Str(String::from("42")));
    match BrewMacro::parse_me(data) {
        Ok(user) => println!("Parsed user: {:?}", user),
        Err(err) => eprintln!("Failed to parse: {}", err),
    }
}

