use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::identity;
use std::ops::Range;
use std::str::FromStr;

use derive_new::new;
use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = PipeMap;
type Input2 = Input;
type Output = u32;

const DAY: u8 = 10;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    let pipe_map = input;
    let index_to_steps = create_pipe_loop(pipe_map);
    *index_to_steps.values().max().unwrap()
}

fn solve2(input: &Input2) -> Output {
    let pipe_map = input;
    let pipe_loop = create_pipe_loop(pipe_map);

    let positions = ray_case_position(pipe_map, &pipe_loop);
    positions.values()
        .filter(|v| **v == Position::Inside)
        .count() as u32
}

fn ray_case_position(
    pipe_map: &PipeMap,
    pipe_loop: &HashMap<PipeIndex, u32>,
) -> HashMap<PipeIndex, Position> {
    let ignore_connections = [Connection::horizontal(), Connection::north_west(), Connection::north_east()];
    let mut can_be_intersected = pipe_loop.iter()
        .map(|(index, _)| (*index, pipe_map.get(index).unwrap()))
        .collect::<HashMap<_, _>>();
    let mut positions = HashMap::new();

    let start_connection = {
        let start = pipe_map.start_position();
        let west = has_connection(pipe_map, &start, Direction::West);
        let east = has_connection(pipe_map, &start, Direction::East);
        let north = has_connection(pipe_map, &start, Direction::North);
        let south = has_connection(pipe_map, &start, Direction::South);
        match (west, north, east, south) {
            (true, false, true, false) => Connection::horizontal(),
            (false, true, false, true) => Connection::vertical(),
            (false, true, true, false) => Connection::north_east(),
            (true, true, false, false) => Connection::north_west(),
            (false, false, true, true) => Connection::south_east(),
            (true, false, false, true) => Connection::south_west(),
            _ => unreachable!(),
        }
    };

    let mut index = pipe_map.start_position();
    can_be_intersected.insert(index, start_connection);

    let mut line_intersections = vec![None; index.width];
    for y in 0..index.height {
        line_intersections.fill(None);
        index = index.new_xy(0, y);
        for x in 0..index.width {
            if let Some(connection) = can_be_intersected.get(&index.new_xy(x, y)) {
                if !ignore_connections.contains(connection) {
                    for i in 0..x {
                        if let Some(value) = line_intersections.get_mut(i).unwrap() {
                            *value += 1;
                        }
                    }
                }
            } else {
                line_intersections[x] = Some(0);
            }
        }
        line_intersections.iter()
            .map(|opt| opt.map(|count| if count % 2 == 1 { Position::Inside } else { Position::Outside }))
            .enumerate()
            .filter_map(|(x, opt)| opt.map(|i| (x, i)))
            .for_each(|(x, position)| { positions.insert(index.new_xy(x, y), position); });
    }
    positions
}

fn has_connection(pipe_map: &PipeMap, from: &PipeIndex, direction: Direction) -> bool {
    from.move_direction(direction)
        .and_then(|i| pipe_map.get(&i))
        .map(|c| c.sides.contains(&direction.opposite()))
        .unwrap_or(false)
}

fn flood_fill(pipe_map: &Input2, pipe_loop: &HashMap<PipeIndex, u32>) {
    let empty_index = create_empty_index(pipe_map, &pipe_loop);
    let mut empty_index_to_position = HashMap::<PipeIndex, Position>::new();

    let mut visited = HashSet::new();
    let mut visit_queue = VecDeque::new();
    for index in empty_index {
        if empty_index_to_position.contains_key(&index) {
            continue;
        }
        let mut is_outside = false;
        visit_queue.push_back(index);
        while !visit_queue.is_empty() {
            let index = visit_queue.pop_front().unwrap();
            visited.insert(index);
            for direction in Direction::ALL {
                let opt_direction_index = index.move_direction(direction);
                if let Some(direction_index) = opt_direction_index {
                    let is_pipe = pipe_map.get(&direction_index).is_some();
                    if is_pipe {
                        if !pipe_loop.contains_key(&direction_index) && !visited.contains(&direction_index) {
                            visit_queue.push_back(direction_index);
                        }
                    } else {
                        if !visited.contains(&direction_index) {
                            visit_queue.push_back(direction_index);
                        }
                    }
                } else {
                    is_outside = true;
                }
            }
        }

        let position = if is_outside { Position::Outside } else { Position::Inside };
        for index in visited.iter() {
            empty_index_to_position.insert(*index, position);
        }
        visited.clear();
        assert!(visit_queue.is_empty());
    }
}

fn cast_ray(
    from: &PipeIndex,
    map: &PipeMap,
    pipe_loop: &HashMap<PipeIndex, u32>,
    ignore: &[Connection],
) -> Position {
    let mut intersections = 0_u32;
    let mut index = *from;
    while let Some(next) = index.move_direction(Direction::West) {
        if pipe_loop.contains_key(&next) && !ignore.contains(&map.get(&next).unwrap()) {
            intersections += 1;
        }
        index = next;
    }
    if intersections % 2 == 1 {
        Position::Inside
    } else {
        Position::Outside
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Position {
    Inside,
    Outside,
}

fn check_position(
    start: &PipeIndex,
    pipe_map: &PipeMap,
    index_to_steps: &HashMap<PipeIndex, u32>,
) -> Position {
    for direction in Direction::ALL {
        let result = move_until(
            start,
            direction,
            |index| pipe_map.get(index).is_some(),
        );
        if let Some(index) = result {
            // we moved to pipe that is not part of a loop
            if !index_to_steps.contains_key(&index) {
                return Position::Outside;
            }
        } else {
            // we moved outside a map
            return Position::Outside;
        }
    }
    // if all 4 sides reached pipe that is part of a loop,
    // then we can say it's inside a loop
    Position::Inside
}

fn move_until(start: &PipeIndex, direction: Direction, end_condition: impl Fn(&PipeIndex) -> bool) -> Option<PipeIndex> {
    let mut index = start.move_direction(direction)?;
    while !end_condition(&index) {
        index = index.move_direction(direction)?;
    }
    Some(index)
}

fn create_empty_index(pipe_map: &PipeMap, pipe_loop: &HashMap<PipeIndex, u32>) -> HashSet<PipeIndex> {
    pipe_map.iter()
        .filter(|(index, c)| match c {
            Some(_) => !pipe_loop.contains_key(index),
            None => true,
        })
        .map(|(index, _)| index)
        .collect()
}

fn create_pipe_loop(pipe_map: &Input) -> HashMap<PipeIndex, u32> {
    let mut index_to_steps = HashMap::<PipeIndex, u32>::new();
    let mut queue = VecDeque::new();
    let start = pipe_map.start_position();
    index_to_steps.insert(start, 0);
    for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
        if let Some((index, direction)) = check_side(direction, &start, &pipe_map, &index_to_steps) {
            index_to_steps.insert(index, 1);
            queue.push_back((index, 1, direction));
        }
    }
    while !queue.is_empty() {
        let (index, steps, direction) = queue.pop_front().unwrap();
        if let Some((move_index, move_direction)) = check_side(direction, &index, &pipe_map, &index_to_steps) {
            index_to_steps.insert(move_index, steps + 1);
            queue.push_back((move_index, steps + 1, move_direction));
        }
    }
    index_to_steps
}

fn check_side(
    direction: Direction,
    from: &PipeIndex,
    pipe_map: &PipeMap,
    index_to_steps: &HashMap<PipeIndex, u32>,
) -> Option<(PipeIndex, Direction)> {
    if let Some(index) = from.move_direction(direction) {
        if !index_to_steps.contains_key(&index) {
            if let Some(connection) = pipe_map.get(&index) {
                if let Some(direction) = connection.connect_from(direction.opposite()) {
                    return Some((index, direction));
                }
            }
        }
    }
    None
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{map, opt, value};
    use nom::multi::many1;
    use nom::sequence::terminated;

    use crate::{Connection, Input, Input2, PipeMap};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let (input, result) = many1(terminated(many1(parse_symbol), opt(line_ending)))(input)?;
        Ok((input, PipeMap::new(result)))
    }


    fn parse_symbol(input: &str) -> IResult<&str, Option<Connection>> {
        alt((
            value(None, tag(".")),
            map(pipe, Some),
        ))(input)
    }

    fn pipe(input: &str) -> IResult<&str, Connection> {
        alt((
            value(Connection::vertical(), tag("|")),
            value(Connection::horizontal(), tag("-")),
            value(Connection::north_east(), tag("L")),
            value(Connection::north_west(), tag("J")),
            value(Connection::south_west(), tag("7")),
            value(Connection::south_east(), tag("F")),
            value(Connection::START, tag("S")),
        ))(input)
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_line() {}

        #[test]
        fn check_all() {}
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct PipeIndex {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl PipeIndex {
    fn new_xy(&self, x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }

    fn move_direction(&self, direction: Direction) -> Option<PipeIndex> {
        match direction {
            Direction::North => {
                if self.y > 0 {
                    return Some(self.new_xy(self.x, self.y - 1));
                }
            }
            Direction::South => {
                if self.y < self.height - 1 {
                    return Some(self.new_xy(self.x, self.y + 1));
                }
            }
            Direction::West => {
                if self.x > 0 {
                    return Some(self.new_xy(self.x - 1, self.y));
                }
            }
            Direction::East => {
                if self.x < self.width - 1 {
                    return Some(self.new_xy(self.x + 1, self.y));
                }
            }
        }
        None
    }

    fn x_range(&self) -> Range<usize> {
        0..self.width
    }

    fn y_range(&self) -> Range<usize> {
        0..self.height
    }
}

#[derive(new, Debug)]
struct PipeMap {
    data: Vec<Vec<Option<Connection>>>,
}

impl PipeMap {
    fn get(&self, index: &PipeIndex) -> Option<Connection> {
        self.data.get(index.y)
            .and_then(|row| {
                row.get(index.x).map(|o| *o).flatten()
            })
    }

    fn start_position(&self) -> PipeIndex {
        let width = self.data[0].len();
        let height = self.data.len();
        self.data.iter()
            .enumerate()
            .filter_map(|(y, row)| {
                let column_result = row.iter()
                    .enumerate()
                    .find(|(x, column)| {
                        if let Some(column) = column {
                            *column == Connection::START
                        } else {
                            false
                        }
                    });
                column_result.map(|(x, _)| {
                    PipeIndex {
                        x,
                        y,
                        width,
                        height,
                    }
                })
            })
            .next().unwrap()
    }

    fn iter(&self) -> impl Iterator<Item=(PipeIndex, Option<Connection>)> + '_ {
        let width = self.data[0].len();
        let height = self.data.len();
        PipeMapIter {
            data: &self.data,
            x: 0,
            y: 0,
            width,
            height,
        }
    }
}

struct PipeMapIter<'a> {
    data: &'a Vec<Vec<Option<Connection>>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for PipeMapIter<'_> {
    type Item = (PipeIndex, Option<Connection>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.height {
            return None;
        }
        let index = PipeIndex {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        };
        let next_x = (self.x + 1) % self.width;
        let next_y = if self.x > next_x { self.y + 1 } else { self.y };
        self.x = next_x;
        self.y = next_y;
        let connection = self.data[index.y][index.x];
        Some((index, connection))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Connection {
    sides: [Direction; 2],
}

impl Connection {
    const START: Connection = Connection { sides: [Direction::North, Direction::North] };

    fn horizontal() -> Self {
        use Direction::*;
        Self { sides: [West, East] }
    }

    fn vertical() -> Self {
        use Direction::*;
        Self { sides: [North, South] }
    }

    fn north_east() -> Self {
        use Direction::*;
        Self { sides: [North, East] }
    }

    fn north_west() -> Self {
        use Direction::*;
        Self { sides: [North, West] }
    }

    fn south_west() -> Self {
        use Direction::*;
        Self { sides: [South, West] }
    }

    fn south_east() -> Self {
        use Direction::*;
        Self { sides: [South, East] }
    }

    fn connect_from(&self, direction: Direction) -> Option<Direction> {
        if self.sides.contains(&direction) {
            for d in self.sides {
                if d != direction {
                    return Some(d);
                }
            }
            None
        } else {
            None
        }
    }
}


fn visualize(input: &str, pipe_map: &PipeMap) {
    let pipe_loop = create_pipe_loop(pipe_map);
    let positions = ray_case_position(pipe_map, &pipe_loop);
    let mut visual = String::new();
    let index = pipe_map.start_position();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let char = if let Some(position) = positions.get(&index.new_xy(x, y)) {
                match position {
                    Position::Inside => 'I',
                    Position::Outside => 'O',
                }
            } else {
                match c {
                    '7' => '╗',
                    'F' => '╔',
                    'J' => '╝',
                    'L' => '╚',
                    '-' => '═',
                    '|' => '║',
                    other => other,
                }
            };
            visual.push(char);
        }
        visual.push('\n');
    }
    println!("{}", visual);
}


#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::{visualize, create_pipe_loop, Position, ray_case_position, solve1, solve2};

    #[test]
    fn square_loop() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        let result = solve1(&output);
        assert_eq!(result, 4);
    }

    #[test]
    fn complex_loop() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        let result = solve1(&output);
        assert_eq!(result, 8);
    }

    #[test]
    fn area_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        visualize(&input, &output);
        let result = solve2(&output);
        assert_eq!(result, 4);
    }

    #[test]
    fn area_1b() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        visualize(&input, &output);
        let result = solve2(&output);
        assert_eq!(result, 4);
    }

    #[test]
    fn area_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        visualize(&input, &output);
        let result = solve2(&output);
        assert_eq!(result, 8);
    }

    #[test]
    fn area_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let (remain, output) = parse(input).unwrap();
        assert_eq!(remain, "");
        visualize(&input, &output);
        let result = solve2(&output);
        assert_eq!(result, 10);
    }
}



