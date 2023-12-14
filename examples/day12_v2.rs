use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;
use std::thread;
use itertools::Itertools;
use memoize::memoize;

use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = Vec<(Vec<SpringStatus>, Vec<usize>)>;
type Input2 = Input;
type Output = usize;

const DAY: u8 = 12;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse, |values| solve2(values));
    execute(DAY, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    input.iter()
        .map(|(status_line, groups)| calculate_v1(&status_line, &groups))
        .sum()
}

fn solve2(input: &Input2) -> Output {
    // let stack_size: usize = 40 * 1024 * 1024;
    // thread::scope(|s| {
    //     let t = thread::Builder::new()
    //         .stack_size(stack_size)
    //         .spawn_scoped(s, || {
    //             input.iter()
    //                 .map(|(status_line, groups)| extend(status_line, groups))
    //                 .map(|(status_line, groups)| calculate_v1(&status_line, &groups))
    //                 .sum()
    //         }).unwrap();
    //     t.join().unwrap()
    // })
    input.par_iter()
        .map(|(status_line, groups)| extend(status_line, groups))
        .map(|(status_line, groups)| calculate_v1(&status_line, &groups))
        .sum()
}

fn extend(status_line: &[SpringStatus], groups: &[usize]) -> (Vec<SpringStatus>, Vec<usize>) {
    let mut line = Vec::with_capacity(status_line.len() * 5 + 4);
    for n in 0..5 {
        if n != 0 {
            line.push(SpringStatus::Unknown);
        }
        for s in status_line.iter() {
            line.push(*s);
        }
    }
    let status_line = line;
    let groups = repeat(groups).take(5).flatten().map(|v| *v).collect_vec();
    (status_line, groups)
}


// TODO: optimize for non recursive solution (DP)
fn calculate_v1(
    status_line: StatusLine,
    groups: Groups,
) -> usize {
    calculate_v1_recursive(status_line, groups, &mut HashMap::new())
}

fn calculate_v1_recursive<'input, 'map>(
    status_line: StatusLine<'input>,
    groups: Groups<'input>,
    mut cache: &'map mut HashMap<(StatusLine<'input>, Groups<'input>), usize>,
) -> usize {
    if status_line.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }
    if groups.is_empty() {
        return if status_line.contains(&SpringStatus::Damaged) { 0 } else { 1 };
    }
    if let Some(result) = cache.get(&(status_line, groups)) {
        return *result;
    }

    let mut result = 0usize;

    if let SpringStatus::Operational | SpringStatus::Unknown = status_line[0] {
        result += calculate_v1_recursive(&status_line[1..], groups, &mut cache);
    }
    if let SpringStatus::Damaged | SpringStatus::Unknown = status_line[0] {
        if has_space_to_fill_group(status_line, groups) &&
            has_no_dots_to_break_group(status_line, groups) &&
            (group_will_finish_line(status_line, groups) ||
                next_is_not_damaged(status_line, groups)) {
            if group_will_finish_line(status_line, groups) {
                result += calculate_v1_recursive(&status_line[groups[0]..], &groups[1..], &mut cache);
            } else {
                result += calculate_v1_recursive(&status_line[groups[0] + 1..], &groups[1..], &mut cache);
            }
        }
    }
    cache.insert((status_line, groups), result);
    result
}

fn has_space_to_fill_group(status_line: StatusLine, groups: Groups) -> bool {
    groups[0] <= status_line.len()
}

fn has_no_dots_to_break_group(status_line: StatusLine, groups: Groups) -> bool {
    !(&status_line[..groups[0]]).contains(&SpringStatus::Operational)
}

fn group_will_finish_line(status_line: StatusLine, groups: Groups) -> bool {
    status_line.len() == groups[0]
}

fn next_is_not_damaged(status_line: StatusLine, groups: Groups) -> bool {
    status_line[groups[0]] != SpringStatus::Damaged
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

type StatusLine<'a> = &'a [SpringStatus];
type Groups<'a> = &'a [usize];


#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use itertools::Itertools;
    use rstest::rstest;
    use tracing::debug;
    use tracing_test::traced_test;

    use crate::{calculate_v1, extend, parser, SpringStatus};

    /*
        n - total amount of spring
        k - amount of damaged in group remained
        t - type of spring (Op = 0, Dmg = 1)

        F(n, k, t)

        F(i, 0, Dmg) = 0 ✅
        F(i, 0, Op) = 1 ✅

        F(1, 1, Dmg) = 1
        F(1, 1, Op) = 0 ✅

        F(2, 1, Dmg) = F(1, 0, Op) ✅
        F(2, 1, Op) = F(1, 1, Dmg)

        F(2, 2, Dmg) = F(1, 1, Dmg)
        F(2, 2, Op) = 0 ✅

        F(3, 1, Dmg) = F(2, 0, Op)
        F(3, 1, Op) = F(2, 1, Dmg) + F(1, 1, Dmg)

        F(3, 2, Dmg) = F(2, 2, Dmg)
        F(3, 2, Op) = F(2, 2, Dmg) * F(1, 2, Dmg)

        F(3, 3, Dmg) = F(2, 2, Dmg) * F(1, 2, Dmg)
        F(3, 3, Op) = 0 ✅

        ////////////////////
        if Op & n > k

        F(2, 1, Op) = F(1, 1, Dmg)
        F(3, 1, Op) = F(2, 1, Dmg) + F(1, 1, Dmg)
        F(4, 1, Op) = F(3, 1, Dmg) + F(2, 1, Dmg) + F(1, 1, Dmg)

        F(3, 2, Op) = F(2, 2, Dmg) * F(1, 1, Dmg) * F(0, 0, Op)
        F(4, 2, Op) = (F(3, 2, Dmg) * F(2, 2, Dmg) * F(1, 2, Op)) + F()

        /////////////////////
        F(i, 0, Dmg) = 0
        F(i, 0, Op) = 1

        F(i, 1, Dmg) = F(i - 1, 0, Op)
        F(i, 1, Op) = F(i - 1, 1, Dmg)

        F(i, i, Op) = 0
        F(i, i, Dmg) = 1


        //////////////////////////////////////////////////////////////////////////
        f(n, k > n, _) = 0
        f(n, n, Op) = 0
        f(n, n, Dmg) = 1

        .#??
        group = 2
        total = 4

        f(4, 2, Op) = 1
        f(4, 2, Dmg) = 1
        f(3, 2, Op) = ?
        f(3, 2, Dmg) = ?
        f(2, 2, Op) = ?
        f(2, 2, Dmg) = f(1, 1, Dmg)
        f(1, 2, Op) = 0
        f(1, 2, Dmg) = 0
        f(0, 2, Op) = 0
        f(0, 2, Dmg) = 0

         */


    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn check_task_1(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        let (rem_input, (status_line, groups)) = parser::parse_line(input).expect("input parsed");
        assert!(rem_input.is_empty(), "input `{input}` is not fully parsed. remained: `rem_input`");
        let actual = calculate_v1(&status_line, &groups);
        assert_eq!(expected, actual, "arrangement count doesn't match for `{input}`");
    }

    #[traced_test]
    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn check_task_2(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        let (rem_input, (status_line, groups)) = parser::parse_line(input).expect("input parsed");
        let (status_line, groups) = extend(&status_line, &groups);
        debug!("{:#?}", status_line);
        debug!("{:?}", groups);
        assert!(rem_input.is_empty(), "input `{input}` is not fully parsed. remained: `rem_input`");
        let actual = calculate_v1(&status_line, &groups);
        assert_eq!(expected, actual, "arrangement count doesn't match for `{input}`");
    }
}


mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::line_ending;
    use nom::combinator::{map, opt, value};
    use nom::IResult;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{separated_pair, terminated};

    use crate::{Input, SpringStatus};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        many1(parse_line)(input)
    }

    pub(crate) fn parse_line(input: &str) -> IResult<&str, (Vec<SpringStatus>, Vec<usize>)> {
        terminated(separated_pair(many1(status), tag(" "), damaged_groups), opt(line_ending))(input)
    }

    fn status(input: &str) -> IResult<&str, SpringStatus> {
        alt((
            value(SpringStatus::Operational, tag(".")),
            value(SpringStatus::Damaged, tag("#")),
            value(SpringStatus::Unknown, tag("?")),
        ))(input)
    }

    fn damaged_groups(input: &str) -> IResult<&str, Vec<usize>> {
        separated_list1(tag(","), map(complete::u32, |v| v as usize))(input)
    }
}
