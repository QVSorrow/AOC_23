use derive_new::new;
use itertools::Itertools;
use rayon::prelude::*;
use tracing::{debug, info, instrument};

use advent_of_code_2023::*;

type Input = Vec<Row>;
type Input2 = Input;
type Output = u32;

const DAY: u8 = 12;

fn main() {
    without_logging();

    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse2, |values| solve2(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    let rows = input;
    rows.iter()
        .map(|row| calculate_arrangements(row))
        .sum()
}

fn solve2(input: &Input2) -> Output {
    input.par_iter()
        .map(|row| expand(row))
        .map(|row| calculate_arrangements(&row))
        .sum()
}

fn expand(row: &Row) -> Row {
    let mut new_statuses = Vec::with_capacity(row.statuses.len() * 5 + 4);
    let mut iter = row.statuses.iter().cycle();
    for i in 0..5 {
        if i != 0 {
            new_statuses.push(None);
        }
        for _ in 0..row.statuses.len() {
            new_statuses.push(*iter.next().unwrap());
        }
    }
    let mut new_groups = Vec::with_capacity(row.damaged_groups.len() * 5);
    let mut iter = row.damaged_groups.iter().cycle();
    for _ in 0..row.damaged_groups.len() * 5 {
        new_groups.push(*iter.next().unwrap());
    }
    Row::new(new_statuses, new_groups)
}

fn calculate_arrangements(row: &Row) -> u32 {
    let unknown_index = row.statuses.iter().enumerate()
        .filter(|(_index, opt)| opt.is_none())
        .map(|(index, _opt)| index)
        .collect_vec();
    let unknown_damaged_size = {
        let total_damaged = row.damaged_groups.iter().sum::<u32>() as usize;
        let known_damaged = row.statuses.iter()
            .filter(|v| v.filter(|s| *s == Status::Damaged).is_some())
            .count();
        total_damaged - known_damaged
    };

    let mut matched = 0_u32;
    let mut unmatched = 0_u32;

    let mut buffer = Vec::with_capacity(unknown_damaged_size);
    check(row, &unknown_index, unknown_damaged_size, &mut buffer, &mut |_slice, is_valid| {
        if is_valid {
            matched += 1;
        } else {
            unmatched += 1;
        }
    });

    info!("Unknown: {}  \t Matched: {}  \t Unmatched: {}", unknown_damaged_size, matched, unmatched);

    matched
}

fn check(
    row: &Row,
    indexes: &[usize],
    remaining: usize,
    used: &mut Vec<usize>,
    result: &mut impl FnMut(&[usize], bool),
) {
    if remaining == 0 {
        let slice = used.as_slice();
        let is_valid = verify(row, slice);
        let row = visualize(row, slice);
        debug!("{row}   -    {}  {slice:?}", if is_valid { "valid" } else { "invalid" });
        result(slice, is_valid);
        return;
    }

    let slice = &indexes[..=indexes.len() - remaining];
    for i in 0..slice.len() {
        used.push(slice[i]);
        check(
            &row,
            &indexes[(i + 1)..],
            remaining - 1,
            used,
            result,
        );
        used.pop();
    }
}

fn visualize(row: &Row, damaged_indexes: &[usize]) -> String {
    let mut output = String::with_capacity(row.statuses.len());

    let mut damaged_index = 0_usize;
    for (index, status) in row.statuses.iter().enumerate() {
        let status = actual_status(damaged_indexes, &mut damaged_index, index, status);
        let c = match status {
            Status::Operational => '.',
            Status::Damaged => '#',
        };
        output.push(c);
    }
    output
}

fn actual_status(damaged_indexes: &[usize], damaged_index: &mut usize, index: usize, status: &Option<Status>) -> Status {
    status.unwrap_or_else(|| {
        if *damaged_index < damaged_indexes.len() && index == damaged_indexes[*damaged_index] {
            *damaged_index += 1;
            Status::Damaged
        } else {
            Status::Operational
        }
    })
}

#[instrument]
fn verify(
    row: &Row,
    damaged_indexes: &[usize],
) -> bool {
    let mut damaged_index = 0usize;
    let statuses = row.statuses.as_slice();
    let groups = row.damaged_groups.as_slice();
    let mut group = 0_usize;
    let mut in_group_count = 0_u32;
    for (index, status) in statuses.iter().enumerate() {
        let status = actual_status(damaged_indexes, &mut damaged_index, index, status);
        match status {
            Status::Operational => {
                if in_group_count > 0 {
                    if in_group_count < groups[group] {
                        // moving to next group, but not filled current
                        return false;
                    }
                    group += 1;
                    in_group_count = 0;
                }
            }
            Status::Damaged => {
                in_group_count += 1;
                if in_group_count > groups[group] {
                    // current group overflow
                    return false;
                }
            }
        }
    }
    if in_group_count > 0 {
        // end with damaged

        // current group must match last
        // current group count must match last group count
        groups.len() == (group + 1) && groups[group] == in_group_count
    } else {
        // end with operational

        // moved to next group, so current group must be one just after last
        // and match groups length
        groups.len() == group
    }
}

#[derive(Debug, Clone, new, Hash)]
struct Row {
    statuses: Vec<Option<Status>>,
    damaged_groups: Vec<u32>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Status {
    Operational,
    Damaged,
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{opt, value};
    use nom::IResult;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{separated_pair, terminated};

    use advent_of_code_2023::integer;

    use crate::{Input, Input2, Row, Status};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        many1(row)(input)
    }

    // ???.### 1,1,3

    fn row(input: &str) -> IResult<&str, Row> {
        let (input, (status, groups)) = terminated(separated_pair(many1(status), tag(" "), damaged_groups), opt(line_ending))(input)?;
        let row = Row::new(status, groups);
        Ok((input, row))
    }

    fn status(input: &str) -> IResult<&str, Option<Status>> {
        alt((
            value(Some(Status::Operational), tag(".")),
            value(Some(Status::Damaged), tag("#")),
            value(None, tag("?")),
        ))(input)
    }

    fn damaged_groups(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(tag(","), integer::<u32>)(input)
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::*;

    use crate::parser::parse;
    use crate::{solve1, solve2};

    #[test]
    fn check_demo() {
        with_info_logging();

        let input = include_str!("day12/demo");
        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve1(&output);
        assert_eq!(result, 21);
    }
    #[test]
    fn check_demo2() {
        with_info_logging();

        let input = include_str!("day12/demo");
        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve2(&output);
        assert_eq!(result, 525152);
    }
}



