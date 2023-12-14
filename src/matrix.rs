use std::fmt::{Display, Formatter};
use std::ops::IndexMut;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    inner: Vec<T>,
    size: Size,
}

impl<T> Matrix<T> where T: Default + Clone {
    pub fn new(size: Size) -> Self {
        let inner = vec![T::default(); size.width() * size.height()];
        Self { inner, size }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, index: Index) -> &T {
        let index = self.index(index);
        self.inner.get(index).unwrap()
    }

    pub fn get_at(&self, x: usize, y: usize) -> &T {
        let index = self.index_xy(x, y);
        self.inner.get(index).unwrap()
    }

    pub fn set(&mut self, index: Index, value: T) {
        let index = self.index(index);
        *self.inner.get_mut(index).unwrap() = value
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: T) {
        let index = self.index_xy(x, y);
        *self.inner.get_mut(index).unwrap() = value
    }

    pub fn size(&self) -> Size {
        self.size
    }

    #[inline(always)]
    fn index_xy(&self, x: usize, y: usize) -> usize {
        self.size.width() * y + x
    }

    #[inline(always)]
    fn index(&self, index: Index) -> usize {
        self.index_xy(index.x(), index.y())
    }
}

impl<T> Display for Matrix<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.height() {
            write!(f, "|| ")?;
            for x in 0..self.size.width() {
                let index = self.index_xy(x, y);
                let value = self.inner.get(index).unwrap();
                write!(f, "{value} |")?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Index(usize, usize);

impl Index {
    #[inline(always)]
    pub fn x(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn y(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size(usize, usize);

impl Size {
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.1
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let size = Size(value[0].len(), value.len());
        let mut inner = value
            .into_iter()
            .flat_map(|i| i.into_iter())
            .collect_vec();
        debug_assert_eq!(size.width() * size.height(), inner.len(), "Size mismatch");
        Self { size, inner }
    }
}

impl<T> From<&[&[T]]> for Matrix<T> where T: Clone {
    fn from(value: &[&[T]]) -> Self {
        let size = Size(value[0].len(), value.len());
        let mut inner = Vec::with_capacity(size.width() * size.height());
        for y in 0..size.height() {
            for x in 0..size.width() {
                inner.push(value[y][x].clone());
            }
        }
        Self { size, inner }
    }
}

impl<T> std::ops::Index<Index> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        let index = self.size.width() * index.y() + index.x();
        self.inner.get(index).unwrap()
    }
}

impl<T> IndexMut<Index> for Matrix<T> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        let index = self.size.width() * index.y() + index.x();
        self.inner.get_mut(index).unwrap()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Column<'a, T> {
    data: &'a Matrix<T>,
    column: usize,
    height: usize,
}

impl<'a, T> Column<'a, T> {
    fn new(data: &'a Matrix<T>, column: usize, height: usize) -> Self {
        Self { data, column, height }
    }

    pub fn x(&self) -> usize {
        self.column
    }

    pub fn len(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        let height = self.height;
        let mut index = 0usize;
        std::iter::from_fn(move || {
            let next = if index < height {
                Some(&self[index])
            } else {
                None
            };
            index += 1;
            next
        })
    }
}

impl<'a, T> PartialEq for Column<'a, T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.height != other.height {
            return false;
        }
        for i in 0..self.height {
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

impl<'a, T> Eq for Column<'a, T> where T: Eq {}

impl<T> std::ops::Index<usize> for Column<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.get_at(self.column, index)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Row<'a, T> {
    data: &'a Matrix<T>,
    row: usize,
    width: usize,
}

impl<'a, T> Row<'a, T> {
    fn new(data: &'a Matrix<T>, row: usize, width: usize) -> Self {
        Self { data, row, width }
    }

    pub fn y(&self) -> usize {
        self.row
    }

    pub fn len(&self) -> usize {
        self.width
    }


    pub fn iter(&self) -> impl Iterator<Item=&T> {
        let width = self.width;
        let mut index = 0usize;
        std::iter::from_fn(move || {
            let next = if index < width {
                Some(&self[index])
            } else {
                None
            };
            index += 1;
            next
        })
    }
}

impl<'a, T> PartialEq for Row<'a, T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width {
            return false;
        }
        for i in 0..self.width {
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

impl<'a, T> Eq for Row<'a, T> where T: Eq {}


impl<T> std::ops::Index<usize> for Row<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.get_at(index, self.row)
    }
}


impl<T> Matrix<T> {
    pub fn column(&self, x: usize) -> Column<'_, T> {
        assert!(x < self.size.width());
        Column::new(self, x, self.size.height())
    }

    pub fn columns(&self) -> impl Iterator<Item=Column<'_, T>> {
        let width = self.size.width();
        let mut index = 0usize;
        std::iter::from_fn(move || {
            let next = if index < width {
                Some(self.column(index))
            } else {
                None
            };
            index += 1;
            next
        })
    }

    pub fn row(&self, y: usize) -> Row<'_, T> {
        assert!(y < self.size.height());
        Row::new(self, y, self.size.width())
    }


    pub fn rows(&self) -> impl Iterator<Item=Row<'_, T>> {
        let height = self.size.height();
        let mut index = 0usize;
        std::iter::from_fn(move || {
            let next = if index < height {
                Some(self.row(index))
            } else {
                None
            };
            index += 1;
            next
        })
    }
}
