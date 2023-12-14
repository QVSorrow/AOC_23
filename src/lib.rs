mod dp_probem;

use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};
use std::iter::Product;
use std::ops::{Div, Mul, Sub};
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

use colored::Colorize;
use nom::{
    combinator::{map_res, recognize},
    error::{make_error, ParseError}, IResult, Parser,
};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{consumed, opt};
use nom::sequence::pair;
use num::{PrimInt, Unsigned};
use tracing::level_filters::LevelFilter;

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
    map_res(consumed(pair(opt(tag("-")), digit1)), |(out, ..)| T::from_str(out))(input)
}

pub fn execute<O, R>(
    day: u8,
    t: Type,
    parse: impl for<'s> Fn(&'s str) -> IResult<&'s str, O>,
    solve: impl Fn(&mut O) -> R,
) where
    R: Display,
{
    let time = Instant::now();
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
    println!("Duration: {}", format!("{:?}", time.elapsed()).red());
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

#[allow(non_snake_case)]
pub fn nCk<N>(n: N, k: N) -> N where N: PrimInt + Unsigned + Product + Copy + Mul + Sub + Div {
    factorial(n) / (factorial(k) * factorial(n - k))
}

#[allow(non_snake_case)]
pub fn nAk<N>(n: N, k: N) -> N where N: PrimInt + Unsigned + Product + Copy + Mul + Sub + Div {
    factorial(n) / factorial(n - k)
}

pub fn factorial<T>(n: T) -> T
    where T: PrimInt + Unsigned + Product
{
    num::range(T::one(), n + T::one()).product()
}

pub fn with_trace_logging() {
    logging(LevelFilter::TRACE)
}

pub fn with_debug_logging() {
    logging(LevelFilter::DEBUG)
}

pub fn with_info_logging() {
    logging(LevelFilter::INFO)
}

pub fn without_logging() {
    logging(LevelFilter::OFF)
}

pub fn logging(level: LevelFilter) {
    color_eyre::install().unwrap();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        // .without_time()
        // .with_target(false)
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}