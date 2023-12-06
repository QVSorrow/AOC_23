use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};
use std::process::exit;
use std::str::FromStr;

use colored::Colorize;
use nom::{
    combinator::{map_res, recognize},
    error::{make_error, ParseError}, IResult, Parser,
};
use nom::character::complete::digit1;

pub fn decimal<T>(input: &str) -> IResult<&str, T>
    where T: FromStr {
    let data = recognize(digit1);
    map_res(data, |s| T::from_str(s))(input)
}

fn read_input(name: &str) -> String {
    let file = File::open(name).unwrap_or_else(|_| {
        eprintln!("\n‼️ Error: There no file {file_name} ‼️ \n\n\
        Possible solutions 🫵: \n\
        \t⚡️ add the input file to {new_file} \n\
        \t⚡️ change your {day} to match Advent of Code challenge day \n\
        \t⚡️ change your {type} to match input file name (\"demo\"/\"input1\"/\"input2\")
        ",
                  file_name = name.red(),
                  new_file = name.yellow(),
                  day = "DAY".blue(),
                  type = "Type".green(),
        );
        exit(-1);
    });
    let mut reader = BufReader::new(file);
    let mut str = String::with_capacity(1024);
    let _ = reader.read_to_string(&mut str);
    str
}

pub enum Type {
    Demo,
    Task1,
    Task2,
}

impl Type {
    pub fn file(&self) -> &'static str {
        match self {
            Type::Demo => "demo",
            Type::Task1 => "input_1",
            Type::Task2 => "input_2",
        }
    }

    pub fn task(&self) -> &'static str {
        match self {
            Type::Demo => "demo",
            Type::Task1 => "1",
            Type::Task2 => "2",
        }
    }
}

pub fn integer<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |out: &str| out.parse::<T>())(input)
}

pub fn execute<O, R>(
    day: u8,
    t: Type,
    parse: impl for<'s> Fn(&'s str) -> IResult<&'s str, O>,
    solve: impl Fn(&mut O) -> R,
) where
    R: Display,
{
    let file_name = format!("examples/day{}/{}", day, t.file());
    let data = read_input(&file_name);
    let (remain, mut parsed) = match parse(&data) {
        Ok(pair) => pair,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    if !remain.is_empty() {
        println!(
            "Day {}. Task {}. Not fully parsed. Remaining",
            day.to_string().blue(),
            t.task().green(),
        );
        println!("{}", remain.yellow());
    }
    let result = solve(&mut parsed);
    println!("Day {}. Task {} -> {}", day.to_string().blue(), t.task().green(), result.to_string().yellow());
}

pub fn window<'i, O, E: ParseError<&'i str>, F>(
    size: usize,
    mut parser: F,
) -> impl FnMut(&'i str) -> IResult<&'i str, O, E>
    where F: Parser<&'i str, O, E>, O: 'i {
    move |input: &'i str| {
        for start in 0..(input.len() - size) {
            let end = start + size;
            let result = parser.parse(&input[start..end]);
            match result {
                Ok((_, o)) => return Ok((input, o)),
                Err(nom::Err::Error(_)) => continue,
                Err(nom::Err::Failure(_)) => return result,
                Err(nom::Err::Incomplete(_)) => continue,
            }
        }
        Err(nom::Err::Error(make_error(input, nom::error::ErrorKind::Fail)))
    }
}