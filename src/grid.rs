use std::{
    collections::{hash_map, HashMap},
    ops::{Index, RangeInclusive},
};

use anyhow::bail;
use itertools::Itertools;

pub struct Grid {
    inner: Vec<Vec<i32>>,

    nrows: usize,
    ncols: usize,
}

impl Grid {
    pub fn new(inner: Vec<Vec<i32>>) -> anyhow::Result<Self> {
        if !inner.iter().map(|v| v.len()).all_equal() {
            bail!("Expected all rows to be of equal len");
        }

        let nrows = inner.len();
        let ncols = inner[0].len();
        Ok(Self {
            inner,
            nrows,
            ncols,
        })
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = i32> {
        self.inner[i].clone().into_iter()
    }

    pub fn iter_col(&self, i: usize) -> impl Iterator<Item = i32> + '_ {
        let mut c = 0;
        std::iter::from_fn(move || {
            let r = if c < self.nrows() {
                Some(self[(i, c)])
            } else {
                None
            };

            c += 1;

            r
        })
    }
}

impl Index<usize> for Grid {
    type Output = Vec<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.inner.index(index.1).index(index.0)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(t: (i32, i32)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}

pub struct InfiniteGrid<T> {
    inner: HashMap<Point, T>,
}

impl<T> InfiniteGrid<T> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub fn entry(&mut self, k: Point) -> hash_map::Entry<'_, Point, T> {
        self.inner.entry(k)
    }

    pub fn get(&self, k: Point) -> Option<&T> {
        self.inner.get(&k)
    }

    pub fn num_points(&mut self) -> usize {
        self.inner.len()
    }

    pub fn dimensions(&self) -> (i32, i32, i32, i32) {
        let min_x = self
            .inner
            .keys()
            .min_by(|p1, p2| p1.x.cmp(&p2.x))
            .unwrap()
            .x;

        let max_x = self
            .inner
            .keys()
            .max_by(|p1, p2| p1.x.cmp(&p2.x))
            .unwrap()
            .x;

        let min_y = self
            .inner
            .keys()
            .min_by(|p1, p2| p1.y.cmp(&p2.y))
            .unwrap()
            .y;

        let max_y = self
            .inner
            .keys()
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .unwrap()
            .y;

        (min_x, max_x, min_y, max_y)
    }
}

impl<T> Default for InfiniteGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl Point {
    pub fn left(&self, x: i32) -> Point {
        debug_assert!(x > 0, "left should be a positive amount");
        (self.x - x, self.y).into()
    }

    pub fn right(&self, x: i32) -> Point {
        debug_assert!(x > 0, "right should be a positive amount");
        (self.x + x, self.y).into()
    }

    pub fn up(&self, y: i32) -> Point {
        debug_assert!(y > 0, "up should be a positive amount");
        (self.x, self.y + y).into()
    }

    pub fn down(&self, y: i32) -> Point {
        debug_assert!(y > 0, "down should be a positive amount");
        (self.x, self.y - y).into()
    }
}
