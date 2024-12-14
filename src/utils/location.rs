use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::{successors, Sum};
use std::ops::{Add, Div, Mul, Neg, RangeInclusive, Rem, Sub};

use num::traits::Euclid;
use num::{one, zero, Bounded, Num, Signed, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
pub struct Location<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Display + Num> Display for Location<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Num> Location<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U: Num, F: Fn(T) -> U>(self, f: F) -> Location<U> {
        Location::new(f(self.x), f(self.y))
    }

    pub fn try_map<U: Num, E, F: Fn(T) -> Result<U, E>>(self, f: F) -> Result<Location<U>, E> {
        Ok(Location::new(f(self.x)?, f(self.y)?))
    }
}

impl<T: Num + Copy + Signed> Location<T> {
    pub fn rotate_90_ccw(self) -> Self {
        Location::new(self.y, -self.x)
    }

    pub fn rotate_90_cw(self) -> Self {
        Location::new(-self.y, self.x)
    }

    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn iter_adjacent(self) -> impl IntoIterator<Item = Location<T>> {
        [
            Location::new(zero(), one::<T>()),
            Location::new(one::<T>(), zero()),
            Location::new(zero(), -one::<T>()),
            Location::new(-one::<T>(), zero()),
        ]
        .map(move |direction| self + direction)
    }
}

impl<T: Num + Copy + PartialOrd> Location<T> {
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            *self + Location::new(zero(), one()),
            *self + Location::new(one(), one()),
            *self + Location::new(one(), zero()),
            *self + Location::new(one(), zero()) - Location::new(zero(), one()),
            *self - Location::new(zero(), one()),
            *self - Location::new(one(), one()),
            *self - Location::new(one(), zero()),
            *self - Location::new(one(), zero()) + Location::new(zero(), one()),
        ]
    }

    pub fn iter_range(self, end: Location<T>) -> SquareIterator<T> {
        SquareIterator {
            next: self,
            next_row: self + Location::new(zero(), one()),
            end,
        }
    }

    pub fn iter_ray(self, direction: Location<T>) -> impl Iterator<Item = Location<T>> {
        successors(Some(self), move |&current| Some(current + direction))
    }

    pub fn square_range(self, end: Location<T>) -> RangeInclusive<Location<T>> {
        self..=(end - Location::new(one(), one()))
    }
}

pub struct SquareIterator<T: Num + Copy + PartialOrd> {
    next: Location<T>,
    next_row: Location<T>,
    end: Location<T>,
}

impl<T: Num + Copy + PartialOrd> Iterator for SquareIterator<T> {
    type Item = Location<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.x < self.end.x {
            let result = self.next;
            self.next = self.next + Location::new(one(), zero());

            return Some(result);
        }

        if self.next_row.y < self.end.y {
            self.next = self.next_row;
            self.next_row = self.next_row + Location::new(zero(), one());

            return self.next();
        }

        None
    }
}

impl<T: Num> Zero for Location<T> {
    fn zero() -> Self {
        Location::new(zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Num> Add<Self> for Location<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num> Sub<Self> for Location<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Location::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Num + Copy> Mul<T> for Location<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Location::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Num + Copy> Div<T> for Location<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Location::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Num + Copy> Div<Self> for Location<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Location::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: Num + Copy> Rem<Self> for Location<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Location::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl<T: Num + Copy + Euclid> Euclid for Location<T> {
    fn div_euclid(&self, rhs: &Self) -> Self {
        Location::new(self.x.div_euclid(&rhs.x), self.y.div_euclid(&rhs.y))
    }

    fn rem_euclid(&self, rhs: &Self) -> Self {
        Location::new(self.x.rem_euclid(&rhs.x), self.y.rem_euclid(&rhs.y))
    }
}

impl<T: Num + Bounded> Bounded for Location<T> {
    fn min_value() -> Self {
        Location::new(T::min_value(), T::min_value())
    }

    fn max_value() -> Self {
        Location::new(T::max_value(), T::max_value())
    }
}

impl<T: Num + Copy + Signed> Neg for Location<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Location::new(-self.x, -self.y)
    }
}

impl<T: Num + Copy> Sum for Location<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Location::zero(), Add::add)
    }
}

// TODO: move elsewhere
pub trait Access2d<T> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T>;
    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()>;
    fn iter_2d_keys(&self) -> SquareIterator<i32>;
}

impl<T> Access2d<T> for Vec<Vec<T>> {
    fn get_2d(&self, loc: Location<i32>) -> Option<&T> {
        self.get(usize::try_from(loc.y).ok()?)
            .and_then(|row| row.get(usize::try_from(loc.x).ok()?))
    }

    fn set_2d(&mut self, loc: Location<i32>, element: T) -> Option<()> {
        self.get_mut(usize::try_from(loc.y).ok()?)
            .and_then(|row| {
                row[usize::try_from(loc.x).ok()?] = element;
                Some(())
            })
            .map(|_| ())
    }

    fn iter_2d_keys(&self) -> SquareIterator<i32> {
        Location::new(0, 0).iter_range(Location::new(self[0].len() as i32, self.len() as i32))
    }
}

impl<T: Num + PartialOrd> PartialOrd for Location<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x < other.x) && (self.y <= other.y) || (self.x <= other.x) && (self.y < other.y) {
            Some(Ordering::Less)
        } else if (self.x > other.x) && (self.y >= other.y) || (self.x >= other.x) && (self.y > other.y) {
            Some(Ordering::Greater)
        } else if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

pub mod direction {
    use crate::utils::location::Location;

    pub const ZERO: Location<i32> = Location::new(0, 0);
    pub const LEFT: Location<i32> = Location::new(-1, 0);
    pub const RIGHT: Location<i32> = Location::new(1, 0);
    pub const UP: Location<i32> = Location::new(0, -1);
    pub const DOWN: Location<i32> = Location::new(0, 1);
}
