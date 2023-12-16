use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::rc::Rc;
use std::str::FromStr;

use rayon::prelude::*;
use tracing::{debug, info};
use tracing::metadata::LevelFilter;

use advent_of_code_2023::{execute, tracing, Type};
use advent_of_code_2023::matrix::Matrix;

type Input = Grid;
type Input2 = Input;
type Output = usize;

const DAY: u8 = 14;

fn main() {
    tracing(LevelFilter::DEBUG);
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse2, |values| solve2(values)); // should be 64
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values)); // should be 102509
}

fn solve1(input: &Input) -> Output {
    let tilted = tilted_north(&input);
    let actual = calculate_north_weight(&tilted);
    actual
}

const ITER_SIZE: usize = 1_000_000_000;

#[derive(Debug, Clone, Eq, PartialEq)]
struct HashedMatrix {
    grid: Grid,
    hash: usize,
}

impl HashedMatrix {
    fn new(grid: Grid) -> Self {
        let hash = calculate_north_weight(&grid);
        Self { grid, hash }
    }
}

impl Hash for HashedMatrix {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.hash)
    }
}

fn solve2(input: &Input2) -> Output {
    let mut cache = HashMap::<Rc<HashedMatrix>, Rc<HashedMatrix>>::new();
    let mut current = Rc::new(HashedMatrix::new(input.clone()));
    let mut contiguous_hits = 0;
    let mut last_iteration = 0_usize;

    for iteration in 1usize..=ITER_SIZE {
        if let Some(matrix) = cache.get(&current) {
            current = matrix.clone();
            contiguous_hits += 1;
        } else {
            contiguous_hits = 0;
            let result = cycle(&current.grid);
            cache.insert(current.clone(), Rc::new(HashedMatrix::new(result)));
        }
        if contiguous_hits > 2 {
            last_iteration = iteration;
            info!("Break after: {} iterations", iteration);
            break;
        }
    }
    let loop_vec = detect_loop(&cache, current.clone());
    debug!("Loop description:");
    for (i, mx) in loop_vec.iter().enumerate() {
        debug!("{}. weight = {}", i + 1, mx.hash);
    }
    let remaining_iterations = ITER_SIZE - last_iteration;
    let loop_index = remaining_iterations % loop_vec.len();
    debug!("remaining cycles: {}", remaining_iterations);
    debug!("remaining loops: {}", remaining_iterations / loop_vec.len());
    debug!("use loop index: {}", loop_index + 1);
    let target = loop_vec[loop_index].clone();
    return target.hash;
}

fn detect_loop(map: &HashMap<Rc<HashedMatrix>, Rc<HashedMatrix>>, from: Rc<HashedMatrix>) -> Vec<Rc<HashedMatrix>> {
    let mut vec = Vec::new();
    vec.push(from.clone());
    let mut current = map.get(&from).unwrap();
    while *current != from {
        vec.push(current.clone());
        current = map.get(current).unwrap();
    }
    vec
}

fn cycle(input: &Grid) -> Grid {
    let input = tilted_north(input);
    let input = tilted_west(&input);
    let input = tilted_south(&input);
    let input = tilted_east(&input);
    input
}

fn tilted_north(input: &Grid) -> Grid {
    let mut target: Matrix<Cell> = Matrix::new(input.size());
    for column in input.columns() {
        let x = column.x();
        let mut next_to_move = 0_usize;
        for y in 0..column.len() {
            let cell = column[y];
            match cell {
                None => continue,
                Some(Rock::Cube) => {
                    next_to_move = y + 1;
                    target.set_at(x, y, cell);
                }
                Some(Rock::Round) => {
                    target.set_at(x, next_to_move, cell);
                    next_to_move += 1;
                }
            }
        }
    }
    target
}

fn tilted_south(input: &Grid) -> Grid {
    let mut target: Matrix<Cell> = Matrix::new(input.size());
    for column in input.columns() {
        let x = column.x();
        let mut next_to_move = column.len() - 1;
        for y in (0..column.len()).rev() {
            let cell = column[y];
            match cell {
                None => continue,
                Some(Rock::Cube) => {
                    target.set_at(x, y, cell);
                    if y == 0 {
                        break;
                    }
                    next_to_move = y - 1;
                }
                Some(Rock::Round) => {
                    target.set_at(x, next_to_move, cell);
                    if next_to_move == 0 {
                        break;
                    }
                    next_to_move -= 1;
                }
            }
        }
    }
    target
}

fn tilted_west(input: &Grid) -> Grid {
    let mut target: Matrix<Cell> = Matrix::new(input.size());
    for row in input.rows() {
        let y = row.y();
        let mut next_to_move = 0_usize;
        for x in 0..row.len() {
            let cell = row[x];
            match cell {
                None => continue,
                Some(Rock::Cube) => {
                    next_to_move = x + 1;
                    target.set_at(x, y, cell);
                }
                Some(Rock::Round) => {
                    target.set_at(next_to_move, y, cell);
                    next_to_move += 1;
                }
            }
        }
    }
    target
}

fn tilted_east(input: &Grid) -> Grid {
    let mut target: Matrix<Cell> = Matrix::new(input.size());
    for row in input.rows() {
        let y = row.y();
        let mut next_to_move = row.len() - 1;
        for x in (0..row.len()).rev() {
            let cell = row[x];
            match cell {
                None => continue,
                Some(Rock::Cube) => {
                    target.set_at(x, y, cell);
                    if x == 0 {
                        break;
                    }
                    next_to_move = x - 1;
                }
                Some(Rock::Round) => {
                    target.set_at(next_to_move, y, cell);
                    if next_to_move == 0 {
                        break;
                    }
                    next_to_move -= 1;
                }
            }
        }
    }
    target
}

fn calculate_north_weight(input: &Grid) -> usize {
    let mut total = 0_usize;
    for column in input.columns() {
        for (index, cell) in column.iter().enumerate() {
            if let Some(Rock::Round) = cell {
                total += column.len() - index;
            }
        }
    }
    total
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Rock {
    Cube,
    Round,
}

type Cell = Option<Rock>;
type Grid = Matrix<Cell>;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{calculate_north_weight, cycle, parser, tilted_north};

    #[rstest]
    #[case("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
    136)]
    fn check_demo_1(
        #[case] input: &str,
        #[case] expected_output: &str,
        #[case] expected: usize,
    ) {
        let (remaining, grid) = parser::parse(input).unwrap();
        assert!(remaining.is_empty());
        let tilted_actual = tilted_north(&grid);
        let (_, tilted_expected) = parser::parse(expected_output).unwrap();
        assert_eq!(tilted_expected, tilted_actual, "transformation incorrect");
        let actual = calculate_north_weight(&tilted_actual);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....")]
    fn check_demo_2_cycle_1(
        #[case] input: &str,
        #[case] expected_output: &str,
    ) {
        let (remaining, grid) = parser::parse(input).unwrap();
        assert!(remaining.is_empty());
        let tilted_actual = cycle(&grid);
        let (_, tilted_expected) = parser::parse(expected_output).unwrap();
        assert_eq!(tilted_expected, tilted_actual, "transformation incorrect");
    }

    #[rstest]
    #[case("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O")]
    fn check_demo_2_cycle_2(
        #[case] input: &str,
        #[case] expected_output: &str,
    ) {
        let (remaining, grid) = parser::parse(input).unwrap();
        assert!(remaining.is_empty());
        let tilted_actual = cycle(&cycle(&grid));
        let (_, tilted_expected) = parser::parse(expected_output).unwrap();
        assert_eq!(tilted_expected, tilted_actual, "transformation incorrect");
    }

    #[rstest]
    #[case("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O")]
    fn check_demo_2_cycle_3(
        #[case] input: &str,
        #[case] expected_output: &str,
    ) {
        let (remaining, grid) = parser::parse(input).unwrap();
        assert!(remaining.is_empty());
        let tilted_actual = cycle(&cycle(&cycle(&grid)));
        let (_, tilted_expected) = parser::parse(expected_output).unwrap();
        assert_eq!(tilted_expected, tilted_actual, "transformation incorrect");
    }
}


mod parser {
    use nom::IResult;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{opt, value};
    use nom::multi::many1;
    use nom::sequence::terminated;

    use advent_of_code_2023::matrix::Matrix;

    use crate::{Cell, Input, Input2, Rock};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let (input, rows) = many1(parse_line)(input)?;
        let mx = Matrix::from(rows);
        Ok((input, mx))
    }

    fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
        terminated(many1(parse_cell), opt(line_ending))(input)
    }

    fn parse_cell(input: &str) -> IResult<&str, Cell> {
        alt((
            value(None, tag(".")),
            value(Some(Rock::Cube), tag("#")),
            value(Some(Rock::Round), tag("O")),
        ))(input)
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }
}



