use std::str::FromStr;
use derive_new::new;
use itertools::{enumerate, Itertools};

use rayon::prelude::*;
use tracing::{debug, trace};
use tracing::metadata::LevelFilter;
use advent_of_code_2023::{execute, tracing, Type};
use advent_of_code_2023::matrix::Matrix;

type Input = Vec<Matrix<Symbol>>;
type Input2 = Input;
type Output = usize;

const DAY: u8 = 13;

fn main() {
    tracing(LevelFilter::INFO);
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse2, |values| solve2(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    input.iter()
        .flat_map(|matrix| find_mirror(matrix, 0))
        .map(|mirror| mirror.sum())
        .sum()
}

fn solve2(input: &Input2) -> Output {
    input.iter()
        .flat_map(|matrix| find_mirror(matrix, 1))
        .map(|mirror| mirror.sum())
        .sum()
}

fn find_mirror(matrix: &Matrix<Symbol>, smudge: usize) -> Vec<Mirror> {
    let mut mirrors = Vec::new();

    let column_mirrors = matrix.columns()
        .enumerate()
        .tuple_windows()
        .filter_map(|((index_left, left), (index_right, right))| {
            let (is_valid, smudges) = is_same_with_smudge(left.iter(), right.iter(), smudge);
            if is_valid {
                Some((index_left, index_right, smudges))
            } else {
                None
            }
        })
        .collect_vec();
    let row_mirrors = matrix.rows()
        .enumerate()
        .tuple_windows()
        .filter_map(|((index_top, top), (index_bottom, bottom))| {
            let (is_valid, smudges) = is_same_with_smudge(top.iter(), bottom.iter(), smudge);
            if is_valid {
                Some((index_top, index_bottom, smudges))
            } else {
                None
            }
        })
        .collect_vec();


    for (mut left, mut right, mut smudge) in column_mirrors {
        let index = left;
        let mut is_valid = true;
        loop {
            if left > 0 && right < matrix.size().width() - 1 {
                left -= 1;
                right += 1;
            } else {
                break;
            }
            let (valid, s) = is_same_with_smudge(matrix.column(left).iter(), matrix.column(right).iter(), smudge);
            if valid {
                trace!("Column #{} change smudge from {} to {}", index, smudge, s);
                smudge = s;
            } else {
                is_valid = false;
                break;
            }
        }
        debug!("Column #{}: valid = {}; smudge = {}", index, is_valid, smudge);
        if is_valid && smudge == 0 {
            mirrors.push(Mirror::Column(index));
        }
    }

    for (mut top, mut bottom, mut smudge) in row_mirrors {
        let index = top;
        let mut is_valid = true;
        loop {
            if top > 0 && bottom < matrix.size().height() - 1 {
                top -= 1;
                bottom += 1;
            } else {
                break;
            }
            let (valid, s) = is_same_with_smudge(matrix.row(top).iter(), matrix.row(bottom).iter(), smudge);
            if valid {
                trace!("Row #{} change smudge from {} to {}", index, smudge, s);
                smudge = s;
            } else {
                is_valid = false;
                break;
            }
        }
        debug!("Row #{}: valid = {}; smudge = {}", index, is_valid, smudge);
        if is_valid && smudge == 0 {
            mirrors.push(Mirror::Row(index));
        }
    }
    mirrors
}

fn is_same_with_smudge<'a>(
    mut a: impl Iterator<Item=&'a Symbol>,
    mut b: impl Iterator<Item=&'a Symbol>,
    smudges: usize,
) -> (bool, usize) {
    let mut rem_smudges = smudges;
    let mut is_equal = true;
    while let Some(a) = a.next() {
        let b = b.next().unwrap();
        if a != b {
            if rem_smudges > 0 {
                trace!("Encountered smudge reduction");
                rem_smudges -= 1;
            } else {
                is_equal = false;
                break
            }
        }
    }
    if is_equal {
        (true, rem_smudges)
    } else {
        (false, smudges)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, new)]
enum Mirror {
    Row(usize),
    Column(usize),
}

impl Mirror {
    fn sum(&self) -> usize {
        match self {
            Mirror::Row(i) => (i + 1) * 100,
            Mirror::Column(i) => i + 1,
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Symbol {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing::metadata::LevelFilter;
    use tracing_test::traced_test;
    use advent_of_code_2023::tracing;

    use crate::{find_mirror, parser, solve1};

    #[traced_test]
    #[rstest]
    #[case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.", 5)]
    #[case("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", 400)]
    fn test_1(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let (rem_input, output) = parser::pattern(input).expect("input parsed");
        assert!(rem_input.is_empty(), "input `{input}` is not fully parsed. remained: `rem_input`");
        let actual = find_mirror(&output, 0).iter().map(|v| v.sum()).sum();
        assert_eq!(expected, actual, "wrong answer");
    }

    #[traced_test]
    #[rstest]
    #[case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.", 300)]
    #[case("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", 100)]
    fn test_2(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let (rem_input, output) = parser::pattern(input).expect("input parsed");
        assert!(rem_input.is_empty(), "input `{input}` is not fully parsed. remained: `rem_input`");
        let actual = find_mirror(&output, 1).iter().map(|v| v.sum()).sum();
        assert_eq!(expected, actual, "wrong answer");
    }
}

mod parser {
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, space0};
    use nom::combinator::{opt, value};
    use nom::IResult;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::terminated;
    use advent_of_code_2023::matrix::Matrix;

    use crate::{Input, Input2, Symbol};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let (input, dataset) = separated_list1(line_ending, pattern)(input)?;
        let output = dataset.into_iter()
            .map(|pattern| Matrix::from(pattern))
            .collect_vec();
        Ok((input, output))
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }

    pub(crate) fn pattern(input: &str) -> IResult<&str, Matrix<Symbol>> {
        let line = terminated(many1(symbol), opt(line_ending));
        let (input, output) = many1(line)(input)?;
        Ok((input, output.into()))
    }

    fn symbol(input: &str) -> IResult<&str, Symbol> {
        alt((
            value(Symbol::Ash, tag(".")),
            value(Symbol::Rock, tag("#")),
        ))(input)
    }
}



