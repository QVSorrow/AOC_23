use std::cmp::max;
use std::ops::{Index, IndexMut};
use tracing::{debug, instrument, trace};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Size(usize, usize);

impl Size {
    pub fn width(&self) -> usize {
        self.0
    }

    pub fn height(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone)]
struct Storage {
    inner: Vec<usize>,
    size: Size,
}

impl Storage {
    fn new(size: Size) -> Self {
        Self { inner: vec![0; size.width() * size.height()], size }
    }

    #[instrument]
    fn get(&mut self, point: Point) -> usize {
        let index = self.point_index(point);
        *self.inner.get(index).unwrap()
    }

    #[instrument]
    fn set(&mut self, point: Point, value: usize) {
        let index = self.point_index(point);
        *self.inner.get_mut(index).unwrap() = value
    }

    #[inline(always)]
    fn index(&self, x: usize, y: usize) -> usize {
        self.size.width() * y + x
    }

    #[inline(always)]
    fn point_index(&self, point: Point) -> usize {
        self.index(point.x(), point.y())
    }

    fn matrix(&self) -> String {
        let mut str = String::new();
        for y in 0..self.size.height() {
            str.push('|');
            str.push('|');
            for x in 0..self.size.width() {
                let index = self.index(x, y);
                let value = self.inner.get(index).unwrap();
                if value > &9 {
                    str.push(' ');
                } else {
                    str.push(' ');
                    str.push(' ');
                }
                str.push_str(&value.to_string());
                str.push(' ');
                str.push('|');
            }
            str.push('|');
            str.push('\n');
        }
        str
    }
}

impl Index<Point> for Storage {
    type Output = usize;

    fn index(&self, index: Point) -> &Self::Output {
        let index = self.size.width() * index.y() + index.x();
        self.inner.get(index).unwrap()
    }
}

impl IndexMut<Point> for Storage {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.size.width() * index.y() + index.x();
        self.inner.get_mut(index).unwrap()
    }
}


///
/// Dynamic programming problem.
///
/// Given a matrix M x N. Find number of distinct ways to reach point (x',y') from point (x, y).
///
/// 1. Define the objective function
///
///         F(x, y) => number of distinct ways to reach Point(x, y)
///
/// 2. Identify base cases
///
///         F(0, 0) = 1
///         F(0, 1) = 1
///         F(1, 0) = 1
///         F(1, 1) = 2
///
/// 3. Write down a recurrence relation for the objective function
///
///         F(x, y) = F(x - 1, y) + F(x, y - 1)
///
/// 4. What the order of execution?
///
///         Bottom-up ?
///
/// 5. Where to for the answer?
///         F(to.x, to.y)
///
///
/// # Arguments
///
/// * `size`: [`Size`] of Matrix
/// * `from`: [`Point`] to start from
/// * `to`: target [`Point`] to come
///
/// returns: [`usize`] numbers of possible path to reach from [`Point`] `from` to [`Point`] `to`
///
fn find_possible_paths(size: Size, from: Point, to: Point) -> usize {
    let mut storage = Storage::new(size);
    storage.set(Point(from.x(), from.y()), 1);

    for y in 0..size.height() {
        for x in 0..size.width() {
            trace!(x, y);
            let left = if x > 0 { storage[Point(x - 1, y)] } else { 0 };
            let right = if x < size.width() - 1 { storage[Point(x + 1, y)] } else { 0 };
            let top = if y < size.height() - 1 { storage[Point(x, y + 1)] } else { 0 };
            let bottom = if y > 0 { storage[Point(x, y - 1)] } else { 0 };
            storage[Point(x, y)] = max(left + top + right + bottom, storage[Point(x, y)]);
        }
    }
    debug!("\n{}", storage.matrix());
    storage[to] // - storage[from] + 1
}


#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use super::Size;
    use super::Point;
    use super::find_possible_paths;


    #[traced_test]
    #[test]
    fn matrix_path() {
        let size = Size(3, 4);
        let from = Point(2, 2);
        let to = Point(2, 3);
        let paths = find_possible_paths(size, from, to);
        println!("{}", paths);
    }
}