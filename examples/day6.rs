use std::str::FromStr;

use derive_new::new;
use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = Vec<Race>;
type Input2 = Race;
type Output = usize;

fn main() {
    execute(6, Type::Demo, parser::parse, |values| solve1(values));
    execute(6, Type::Task1, parser::parse, |values| solve1(values));
    execute(6, Type::Demo, parser::parse2, |values| solve2(values));
    execute(6, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    input.iter()
        .map(|race| {
            (1..race.time).into_iter()
                .map(|charge| race_distance(race.time, charge))
                .filter(|distance| *distance > race.record)
                .count()
        })
        .product::<usize>()
}

fn solve2(input: &Input2) -> Output {
    let race = input;
    (1..race.time).into_iter()
        .map(|charge| race_distance(race.time, charge))
        .filter(|distance| *distance > race.record)
        .count()
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, line_ending};
    use nom::combinator::{map_res, opt};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, pair};

    use advent_of_code_2023::integer;

    use crate::{Distance, Input, Input2, Race, Time};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let (input, time) = delimited(
            pair(tag("Time:"), many1(tag(" "))),
            separated_list1(many1(tag(" ")), integer::<Time>),
            opt(line_ending),
        )(input)?;
        let (input, distances) = delimited(
            pair(tag("Distance:"), many1(tag(" "))),
            separated_list1(many1(tag(" ")), integer::<Distance>),
            opt(line_ending),
        )(input)?;

        let races = time.into_iter().zip(distances.into_iter())
            .map(|(time, distance)| Race::new(time, distance))
            .collect();

        Ok((input, races))
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        let (input, time) = map_res(
            delimited(
                pair(tag("Time:"), many1(tag(" "))),
                separated_list1(many1(tag(" ")), digit1),
                opt(line_ending),
            ), |vec| make_number::<Time>(&vec))(input)?;
        let (input, distance) = map_res(
            delimited(
                pair(tag("Distance:"), many1(tag(" "))),
                separated_list1(many1(tag(" ")), digit1),
                opt(line_ending),
            ), |vec| make_number::<Distance>(&vec))(input)?;

        let race = Race::new(time, distance);

        Ok((input, race))
    }

    fn make_number<T: FromStr>(data: &[&str]) -> Result<T, T::Err> {
        let mut str = String::new();
        for &s in data {
            str.push_str(s);
        }
        str.parse::<T>()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_line() {}

        #[test]
        fn check_all() {}
    }
}


type Time = u64;
type Distance = u64;


fn race_distance(time: Time, charge: Time) -> Distance {
    let remained_time = time - charge;
    remained_time * charge
}

#[derive(Debug, Copy, Clone, new)]
struct Race {
    time: Time,
    record: Distance,
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {}
}



