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


use brewfile_derive::CmdParser;

#[derive(CmdParser, Debug)]
struct BrewMacro {
    first: String,

    #[arg]
    second: String,

    third: u32,

    fourth: Vec<u32>,

    fifth: Vec<String>,
}

#[derive(Debug)]
struct Test {
    first: String,
}


impl Test {
    fn new() -> Self {
        let mut test = Self{ first: String::new() };
        test.first = String::from("Hello");
        test
    }
}

fn main() {

    println!("Hello world");

    let data = vec![
        brewfile_parser::ast::Ident::Str(String::from("hello")),
        brewfile_parser::ast::Ident::Str(String::from("world")),
        brewfile_parser::ast::Ident::Str(String::from("42")),
        brewfile_parser::ast::Ident::List(vec![String::from("42")]),
        brewfile_parser::ast::Ident::List(vec![String::from("42")]),
    ];
    match BrewMacro::parse_me(data) {
        Ok(user) => println!("Parsed user: {:?}", user),
        Err(err) => eprintln!("Failed to parse: {}", err),
    };

    let data = vec![
        brewfile_parser::ast::Ident::Str(String::from("hello")),
        brewfile_parser::ast::Ident::Str(String::from("world")),
        brewfile_parser::ast::Ident::Str(String::from("42")),
        brewfile_parser::ast::Ident::List(vec![String::from("42")]),
        brewfile_parser::ast::Ident::List(vec![String::from("42")]),
        brewfile_parser::ast::Ident::Named(String::from("first"), Box::new(brewfile_parser::ast::Ident::Str(String::from("hello")))),
    ];
    println!("----- brew {:#?}", BrewMacro::parse_ident(data));

    let mut map: HashMap<String, brewfile_parser::ast::Ident> = HashMap::new();
    let data = brewfile_parser::ast::Ident::Named(String::from("first"), Box::new(brewfile_parser::ast::Ident::Str(String::from("hello world"))));

    match data {
        brewfile_parser::ast::Ident::Named(key, val) => {
            map.insert(key.to_owned(), brewfile_parser::ast::Ident::Str(String::from("hello")));


            let val = *val;
            map.insert(key.to_owned(), val);
        },
        _ => panic!("WTF"),
    }

    println!("----- brew {:#?}", map);
}

