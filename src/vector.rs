use std::ops::{Add, Div, Mul, Rem, Sub};
use std::vec;

trait VectorTrait:
    Clone
    + Copy
    + Default
    + core::fmt::Debug
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
        + core::fmt::Debug
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_normalized_at_initialization() {
        let v1 = Vector::from(vec![0; 0]);
        let v2 = Vector::from(&[0; 1] as &[i32]);
        let v3 = Vector::from(vec![0; 2]);
        let v4 = Vector::from(vec![1, 2, 3, 0]);
        let v5 = Vector::from(&[1, 2, 0, 0] as &[i32]);
        let v6 = Vector::from(vec![1, 0, 3, 0]);
        let v7 = Vector::from(&[0, 0, 3, 0] as &[i32]);
        let v8 = Vector::from(vec![1, 2, 3, 4]);

        assert_eq!(v1.inner, &[]);
        assert_eq!(v2.inner, &[]);
        assert_eq!(v3.inner, &[]);
        assert_eq!(v4.inner, &[1, 2, 3]);
        assert_eq!(v5.inner, &[1, 2]);
        assert_eq!(v6.inner, &[1, 0, 3]);
        assert_eq!(v7.inner, &[0, 0, 3]);
        assert_eq!(v8.inner, &[1, 2, 3, 4]);
    }

    #[test]
    fn from_vec() {
        let v1 = Vector::from(vec![0; 0]);
        let v2 = Vector::from(vec![1, 2, 3]);
        let v3 = Vector::from(vec![1, 2, 3, 4]);

        assert_eq!(v1.inner, &[]);
        assert_eq!(v2.inner, &[1, 2, 3]);
        assert_eq!(v3.inner, &[1, 2, 3, 4]);
    }

    #[test]
    fn from_slice() {
        let v1 = Vector::from(&[0; 0] as &[i32]);
        let v2 = Vector::from(&[1, 2, 3] as &[i32]);
        let v3 = Vector::from(&[1, 2, 3, 4] as &[i32]);

        assert_eq!(v1.inner, &[]);
        assert_eq!(v2.inner, &[1, 2, 3]);
        assert_eq!(v3.inner, &[1, 2, 3, 4]);
    }

    #[test]
    fn to_vec() {
        let s1 = vec![0; 0];
        let s2 = vec![1, 2, 3];
        let s3 = vec![1, 2, 3, 4];
        let v1 = Vector { inner: s1.clone() };
        let v2 = Vector { inner: s2.clone() };
        let v3 = Vector { inner: s3.clone() };

        assert_eq!(v1.to_vec(), s1);
        assert_eq!(v2.to_vec(), s2);
        assert_eq!(v3.to_vec(), s3);
    }

    #[test]
    fn as_slice() {
        let s1: &[i32] = &[0; 0];
        let s2: &[i32] = &[1, 2, 3];
        let s3: &[i32] = &[1, 2, 3, 4];
        let v1 = Vector { inner: s1.to_vec() };
        let v2 = Vector { inner: s2.to_vec() };
        let v3 = Vector { inner: s3.to_vec() };

        assert_eq!(v1.as_slice(), s1);
        assert_eq!(v2.as_slice(), s2);
        assert_eq!(v3.as_slice(), s3);
    }

    #[test]
    fn normalize() {
        let mut v1 = Vector { inner: vec![0; 0] };
        let mut v2 = Vector { inner: vec![0; 1] };
        let mut v3 = Vector { inner: vec![0; 2] };
        let mut v4 = Vector {
            inner: vec![1, 2, 3, 0],
        };
        let mut v5 = Vector {
            inner: vec![1, 2, 0, 0],
        };
        let mut v6 = Vector {
            inner: vec![1, 0, 3, 0],
        };
        let mut v7 = Vector {
            inner: vec![0, 0, 3, 0],
        };
        let mut v8 = Vector {
            inner: vec![1, 2, 3, 4],
        };

        v1.normalize();
        v2.normalize();
        v3.normalize();
        v4.normalize();
        v5.normalize();
        v6.normalize();
        v7.normalize();
        v8.normalize();

        assert_eq!(v1.inner, &[]);
        assert_eq!(v2.inner, &[]);
        assert_eq!(v3.inner, &[]);
        assert_eq!(v4.inner, &[1, 2, 3]);
        assert_eq!(v5.inner, &[1, 2]);
        assert_eq!(v6.inner, &[1, 0, 3]);
        assert_eq!(v7.inner, &[0, 0, 3]);
        assert_eq!(v8.inner, &[1, 2, 3, 4]);
    }

    #[test]
    fn add() {
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) + Vector::from(vec![1, 2, 3, 4]),
            Vector::from(vec![2, 4, 6, 8])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) + Vector::from(vec![]),
            Vector::from(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Vector::<i32>::from(vec![]) + Vector::from(vec![]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![1]) + Vector::from(vec![]),
            Vector::from(vec![1])
        );
        assert_eq!(
            Vector::from(vec![]) + Vector::from(vec![1]),
            Vector::from(vec![1])
        );
        assert_eq!(
            Vector::from(vec![]) + Vector::from(vec![1, 2]),
            Vector::from(vec![1, 2])
        );
        assert_eq!(
            Vector::from(vec![1, 2]) + Vector::from(vec![]),
            Vector::from(vec![1, 2])
        );
        assert_eq!(
            Vector::from(vec![1, 0, 1, 0]) + Vector::from(vec![0, -1, 0, -1]),
            Vector::from(vec![1, -1, 1, -1])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) + Vector::from(vec![1, 2, 3]),
            Vector::from(vec![2, 4, 6, 4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) + Vector::from(vec![1, 2, 3, 4]),
            Vector::from(vec![2, 4, 6, 4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) + Vector::from(vec![1, -2, -3]),
            Vector::from(vec![2])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) + Vector::from(vec![-1, -2, -3]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![10, -5, 100, -50]) + Vector::from(vec![50, 10, -40, -50]),
            Vector::from(vec![60, 5, 60, -100])
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) - Vector::from(vec![1, 2, 3, 4]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) - Vector::from(vec![]),
            Vector::from(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Vector::<i32>::from(vec![]) - Vector::from(vec![]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![1]) - Vector::from(vec![]),
            Vector::from(vec![1])
        );
        assert_eq!(
            Vector::from(vec![]) - Vector::from(vec![1]),
            Vector::from(vec![-1])
        );
        assert_eq!(
            Vector::from(vec![]) - Vector::from(vec![1, 2]),
            Vector::from(vec![-1, -2])
        );
        assert_eq!(
            Vector::from(vec![1, 2]) - Vector::from(vec![]),
            Vector::from(vec![1, 2])
        );
        assert_eq!(
            Vector::from(vec![1, 0, 1, 0]) - Vector::from(vec![0, -1, 0, -1]),
            Vector::from(vec![1, 1, 1, 1])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) - Vector::from(vec![1, 2, 3]),
            Vector::from(vec![0, 0, 0, 4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) - Vector::from(vec![1, 2, 3, 4]),
            Vector::from(vec![0, 0, 0, -4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) - Vector::from(vec![1, -2, -3]),
            Vector::from(vec![0, 4, 6])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3]) - Vector::from(vec![-1, -2, -3]),
            Vector::from(vec![2, 4, 6])
        );
        assert_eq!(
            Vector::from(vec![10, -5, 100, -50]) - Vector::from(vec![50, 10, -40, -50]),
            Vector::from(vec![-40, -15, 140])
        );
    }

    #[test]
    fn mul_t() {
        assert_eq!(Vector::from(vec![1, 2, 3, 4]) * 0, Vector::from(vec![]));
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) * 1,
            Vector::from(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) * -1,
            Vector::from(vec![-1, -2, -3, -4])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) * 10,
            Vector::from(vec![10, 20, 30, 40])
        );
        assert_eq!(
            Vector::from(vec![1, 2, 3, 4]) * -10,
            Vector::from(vec![-10, -20, -30, -40])
        );
    }

    #[test]
    fn div() {
        assert_eq!(
            Vector::from(vec![1]) / Vector::from(vec![1]),
            Vector::from(vec![1])
        );
        assert_eq!(
            Vector::from(vec![1, 2]) / Vector::from(vec![1]),
            Vector::from(vec![1, 2])
        );
        assert_eq!(
            Vector::from(vec![1]) / Vector::from(vec![1, 2]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![-1, 1, 2, 0, 1]) / Vector::from(vec![1, 1, 1]),
            Vector::from(vec![2, -1, 1])
        );
        assert_eq!(
            Vector::from(vec![-2, -3, -8, 4]) / Vector::from(vec![-3, 2]),
            Vector::from(vec![-3, -1, 2])
        );
        assert_eq!(
            Vector::from(vec![8, 0, -14, 0, 3]) / Vector::from(vec![1, -2, 1]),
            Vector::from(vec![-5, 6, 3])
        );
        assert_eq!(
            Vector::from(vec![-4, 19, -24, -1, 10]) / Vector::from(vec![1, -3, 2]),
            Vector::from(vec![-4, 7, 5])
        );
    }

    #[test]
    fn rem() {
        assert_eq!(
            Vector::from(vec![1]) % Vector::from(vec![1]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![1, 2]) % Vector::from(vec![1]),
            Vector::from(vec![])
        );
        assert_eq!(
            Vector::from(vec![1]) % Vector::from(vec![1, 2]),
            Vector::from(vec![1])
        );
        assert_eq!(
            Vector::from(vec![-1, 1, 2, 0, 1]) % Vector::from(vec![1, 1, 1]),
            Vector::from(vec![-3])
        );
        assert_eq!(
            Vector::from(vec![-2, -3, -8, 4]) % Vector::from(vec![-3, 2]),
            Vector::from(vec![-11])
        );
        assert_eq!(
            Vector::from(vec![8, 0, -14, 0, 3]) % Vector::from(vec![1, -2, 1]),
            Vector::from(vec![13, -16])
        );
        assert_eq!(
            Vector::from(vec![-4, 19, -24, -1, 10]) % Vector::from(vec![1, -3, 2]),
            Vector::from(vec![])
        );
    }

    #[test]
    #[should_panic]
    fn div_zero() {
        let _ = Vector::from(vec![1]) / Vector::from(vec![]);
    }
}
