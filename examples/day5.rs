use std::ops::Range;
use std::str::FromStr;

use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = (Almanac, Seeds);
type Output = u64;

fn main() {
    execute(5, Type::Demo, parser::parse, |values| solve1(values));
    execute(5, Type::Task1, parser::parse, |values| solve1(values));
    execute(5, Type::Demo, parser::parse, |values| solve2(values));
    execute(5, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    let (almanac, seeds) = input;
    let min_location = seeds.into_iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .expect("Any seeds present");
    min_location
}

fn solve2(input: &Input) -> Output {
    let almanac = &input.0;
    let min_location = input.1.chunks_exact(2)
        .map(|arr| (arr[0], arr[1]))
        .par_bridge()
        .flat_map(|(start, length)| start..(start + length))
        .map(|seed| almanac.seed_to_location(seed))
        .min().expect("Any seeds present");
    min_location
}

mod parser {
    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{map, opt, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, pair, terminated, tuple};

    use crate::{Almanac, AlmanacMap, Input, integer, Seeds};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let parsers = (terminated(parse_seeds, empty_line),
                       terminated(parse_map("seed-to-soil"), empty_line),
                       terminated(parse_map("soil-to-fertilizer"), empty_line),
                       terminated(parse_map("fertilizer-to-water"), empty_line),
                       terminated(parse_map("water-to-light"), empty_line),
                       terminated(parse_map("light-to-temperature"), empty_line),
                       terminated(parse_map("temperature-to-humidity"), empty_line),
                       terminated(parse_map("humidity-to-location"), empty_line),
        );
        let (input, results) = tuple(parsers).parse(input)?;
        let (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ) = results;

        let almanac = Almanac {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };

        println!("Parsed almanac");
        Ok((input, (almanac, seeds)))
    }

    fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
        println!("Parsing seeds");
        delimited(
            tag("seeds: "),
            separated_list1(tag(" "), integer),
            opt(line_ending),
        )(input)
    }

    fn empty_line(input: &str) -> IResult<&str, ()> {
        // value((), terminated(take_till(|c| c == '\n' || c == '\r'), opt(line_ending)))(input)
        value((), opt(line_ending))(input)
    }

    fn parse_map(name: &str) -> impl for<'parser> FnMut(&'parser str) -> IResult<&'parser str, AlmanacMap> + '_ {
        move |input| {
            let (input, _) = terminated(pair(tag(name), tag(" map:")), line_ending)(input)?;

            let entry = map(
                terminated(
                    tuple((integer, tag(" "), integer, tag(" "), integer)),
                    opt(line_ending),
                ), |(destination, _, source, _, length)| Entry {
                    source,
                    destination,
                    length,
                });
            let (input, entries) = many1(entry)(input)?;

            let mut map = AlmanacMap::default();

            for Entry { source, destination, length } in entries {
                map.add(source, destination, length);
            }
            println!("Parsed {} map", name);
            Ok((input, map))
        }
    }

    struct Entry {
        source: u64,
        destination: u64,
        length: u64,
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_line() {}

        #[test]
        fn check_all() {}
    }
}


fn integer<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |out: &str| out.parse::<T>())(input)
}

type Seeds = Vec<u64>;

#[derive(Debug, Clone)]
struct Almanac {
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);
        location
    }

    fn debug_seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);
        println!("seed({seed}) -> soil({soil}) -> fertilizer({fertilizer}) -> water({water}) -> light({light}) -> temperature({temperature}) -> humidity({humidity}) -> location({location})");
        location
    }
}

#[derive(Debug, Clone, Default)]
struct AlmanacMap {
    vec: Vec<(Range<u64>, Range<u64>)>,
}

impl AlmanacMap {
    fn add(&mut self, from: u64, to: u64, len: u64) {
        self.vec.push((from..(from + len), to..(to + len)));
    }

    fn map(&self, from: u64) -> u64 {
        let range = self.vec.iter().find(|(range, ..)| range.contains(&from));
        if let Some((source, destination)) = range {
            let index = from - source.start;
            destination.start + index
        } else {
            from
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {}
}



