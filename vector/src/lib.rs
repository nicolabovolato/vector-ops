use std::ops::{Add, Div, Mul, Rem, Sub};
use std::vec;

trait VectorTrait:
    Clone
    + Copy
    + Default
    + std::cmp::PartialEq
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
{
}

impl<T> VectorTrait for T where
    T: Clone
        + Copy
        + Default
        + std::cmp::PartialEq
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
{
}

#[derive(Debug, Clone, PartialEq)]
struct Vector<T: VectorTrait> {
    inner: Vec<T>,
}

impl<T: VectorTrait> Vector<T> {
    fn normalize(&mut self) {
        if self.inner.len() <= 0 {
            return;
        }

        for i in (0..self.inner.len()).rev() {
            if self.inner[i] != T::default() {
                return;
            }
            self.inner.pop();
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.inner.clone()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<T: VectorTrait> From<&[T]> for Vector<T> {
    fn from(slice: &[T]) -> Self {
        let mut v = Self {
            inner: slice.to_vec(),
        };
        v.normalize();
        v
    }
}

impl<T: VectorTrait> From<Vec<T>> for Vector<T> {
    fn from(slice: Vec<T>) -> Self {
        let mut v = Self { inner: slice };
        v.normalize();
        v
    }
}

impl<T: VectorTrait> Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let l1 = self.inner.len();
        let l2 = rhs.inner.len();
        let mut addend1 = self.inner.to_vec();
        let mut addend2 = rhs.inner.to_vec();

        if l1 > l2 {
            addend2.extend(vec![T::default(); l1 - l2]);
        } else if l2 > l1 {
            addend1.extend(vec![T::default(); l2 - l1]);
        }

        let mut result: Vec<T> = Vec::new();
        for (val1, val2) in addend1.iter().zip(addend2.iter()) {
            result.push(*val1 + *val2);
        }

        let mut v = Self::Output::from(result);
        v.normalize();
        v
    }
}

impl<T: VectorTrait> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let l1 = self.inner.len();
        let l2 = rhs.inner.len();
        let mut minuend = self.inner.to_vec();
        let mut subtrahend = rhs.inner.to_vec();

        if l1 > l2 {
            subtrahend.extend(vec![T::default(); l1 - l2]);
        } else if l2 > l1 {
            minuend.extend(vec![T::default(); l2 - l1]);
        }

        let mut result: Vec<T> = Vec::new();
        for (val1, val2) in minuend.iter().zip(subtrahend.iter()) {
            result.push(*val1 - *val2);
        }

        let mut v = Self::Output::from(result);
        v.normalize();
        v
    }
}

impl<T: VectorTrait> Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut v3: Vec<T> = Vec::new();

        for val1 in self.inner.iter() {
            v3.push(*val1 * rhs);
        }

        let mut v = Self::Output::from(v3);
        v.normalize();
        v
    }
}

impl<T: VectorTrait> Vector<T> {
    fn div_inner(&self, divisor: Self) -> (Self, Self) {
        let mut dividend_mut = Self::from(self.inner.clone());
        let mut result = Self::from(vec![T::default(); 0]);

        if divisor.inner.len() == 0 {
            panic!("division by empty vector");
        }

        for _ in 0..dividend_mut.inner.len() {
            let last_dividend = *dividend_mut.inner.last().unwrap_or(&T::default());
            let last_divisor = *divisor.inner.last().unwrap_or(&T::default());

            let coefficient = last_dividend / last_divisor;

            if divisor.inner.len() > dividend_mut.inner.len() || coefficient == T::default() {
                return (result, dividend_mut);
            }

            let grade = dividend_mut.inner.len() - divisor.inner.len();

            let mut div_v = vec![T::default(); grade];
            div_v.extend((divisor.clone() * coefficient).inner);

            dividend_mut = dividend_mut - Self::from(div_v);
            result.inner.insert(0, coefficient);
            dividend_mut.normalize();
        }

        (result, dividend_mut)
    }
}

impl<T: VectorTrait> Div for Vector<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div_inner(rhs).0
    }
}

impl<T: VectorTrait> Rem for Vector<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.div_inner(rhs).1
    }
}
