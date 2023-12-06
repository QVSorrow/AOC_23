use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};
use std::str::FromStr;

use nom::{
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::many1,
    IResult, Parser, error::{make_error, ParseError},
};
use nom::character::complete::digit1;

#[deprecated(note="please use `decimal<T>` instead")]
pub fn decimal_legacy(input: &str) -> IResult<&str, u32> {
    let num_condition = many1(one_of("0123456789"));
    let recognize = recognize(num_condition);
    map_res(recognize, |out: &str| out.parse::<u32>()).parse(input)
}

pub fn decimal<T>(input: &str) -> IResult<&str, T>
where T : FromStr {
    let data = recognize(digit1);
    map_res(data, |s| T::from_str(s))(input)
}

fn read_input(name: &str) -> String {
    let file = File::open(name).unwrap();
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
            day,
            t.task()
        );
        println!("{}", remain);
    }
    let result = solve(&mut parsed);
    println!("Day {}. Task {} -> {}", day, t.task(), result);
}

pub fn window<'i, O, E: ParseError<&'i str>, F>(
    size: usize,
    mut parser: F,
) -> impl FnMut(&'i str) -> IResult<&'i str, O, E>
    where F: Parser<&'i str, O, E>, O : 'i {
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