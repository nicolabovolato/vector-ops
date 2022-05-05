use std::vec;

fn div_inner<T>(dividend: &[T], divisor: &[T]) -> (Vec<T>, Vec<T>)
where
    T: Clone
        + Copy
        + Default
        + std::cmp::PartialEq
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    let mut dividend_mut = dividend.to_vec();
    let mut rest = vec![T::default(); 0];

    if divisor.len() == 0 {
        panic!("division by empty vector");
    }

    for _ in 0..dividend_mut.len() {
        let last_v3 = *dividend_mut.last().unwrap_or(&T::default());
        let last_v2 = *divisor.last().unwrap_or(&T::default());

        let coefficient = last_v3 / last_v2;

        if divisor.len() > dividend_mut.len() || coefficient == T::default() {
            return (rest, dividend_mut);
        }

        let grade = dividend_mut.len() - divisor.len();

        let mut div_v = vec![T::default(); grade];
        div_v.extend(mul_scalar(divisor, coefficient));

        dividend_mut = sub(&dividend_mut, &div_v);
        rest.insert(0, coefficient);
        normalize(&mut dividend_mut);
    }

    (rest, dividend_mut)
}

fn div<T>(dividend: &[T], divisor: &[T]) -> Vec<T>
where
    T: Clone
        + Copy
        + Default
        + std::cmp::PartialEq
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    let d = div_inner(dividend, divisor);
    return d.0;
}

fn rem<T>(dividend: &[T], divisor: &[T]) -> Vec<T>
where
    T: Clone
        + Copy
        + Default
        + std::cmp::PartialEq
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    return div_inner(dividend, divisor).1;
}

fn mul_scalar<T>(multiplicand: &[T], multiplier: T) -> Vec<T>
where
    T: Clone + Copy + Default + std::ops::Mul<Output = T>,
{
    let mut v3: Vec<T> = Vec::new();

    for val1 in multiplicand.iter() {
        v3.push(*val1 * multiplier);
    }

    v3
}

fn add<T>(addend1: &[T], addend2: &[T]) -> Vec<T>
where
    T: Clone + Copy + Default + std::cmp::PartialEq + std::ops::Add<Output = T>,
{
    let l1 = addend1.len();
    let l2 = addend2.len();
    let mut addend1 = addend1.to_vec();
    let mut addend2 = addend2.to_vec();

    if l1 > l2 {
        addend2.extend(vec![T::default(); l1 - l2]);
    } else if l2 > l1 {
        addend1.extend(vec![T::default(); l2 - l1]);
    }

    let mut result: Vec<T> = Vec::new();
    for (val1, val2) in addend1.iter().zip(addend2.iter()) {
        result.push(*val1 + *val2);
    }

    normalize(&mut result);
    result
}

fn sub<T>(minuend: &[T], subtrahend: &[T]) -> Vec<T>
where
    T: Clone + Copy + Default + std::cmp::PartialEq + std::ops::Sub<Output = T>,
{
    let l1 = minuend.len();
    let l2 = subtrahend.len();
    let mut minuend = minuend.to_vec();
    let mut subtrahend = subtrahend.to_vec();

    if l1 > l2 {
        subtrahend.extend(vec![T::default(); l1 - l2]);
    } else if l2 > l1 {
        minuend.extend(vec![T::default(); l2 - l1]);
    }

    let mut result: Vec<T> = Vec::new();
    for (val1, val2) in minuend.iter().zip(subtrahend.iter()) {
        result.push(*val1 - *val2);
    }

    normalize(&mut result);
    result
}

fn normalize<T>(v: &mut Vec<T>)
where
    T: Clone + Copy + Default + std::cmp::PartialEq,
{
    if v.len() <= 0 {
        return;
    }

    for i in (0..v.len()).rev() {
        if v[i] != T::default() {
            return;
        }
        v.pop();
    }
}

fn longest<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: Clone,
{
    if v1.len() >= v2.len() {
        v1.to_vec()
    } else {
        v2.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let v1 = vec![0; 1];
        let v2 = vec![0; 2];
        let v3 = vec![0; 3];
        let v4 = vec![0; 4];
        let v5 = vec![0; 5];

        assert_eq!(longest(&v1, &v2), v2);
        assert_eq!(longest(&v2, &v3), v3);
        assert_eq!(longest(&v3, &v4), v4);
        assert_eq!(longest(&v4, &v5), v5);
    }

    #[test]
    fn test_normalize() {
        let mut v1 = vec![0; 0];
        let mut v2 = vec![0; 1];
        let mut v3 = vec![0; 2];
        let mut v4 = vec![1, 2, 3, 0];
        let mut v5 = vec![1, 2, 0, 0];
        let mut v6 = vec![1, 0, 3, 0];
        let mut v7 = vec![0, 0, 3, 0];
        let mut v8 = vec![1, 2, 3, 4];

        normalize(&mut v1);
        normalize(&mut v2);
        normalize(&mut v3);
        normalize(&mut v4);
        normalize(&mut v5);
        normalize(&mut v6);
        normalize(&mut v7);
        normalize(&mut v8);

        assert_eq!(v1.as_slice(), &[]);
        assert_eq!(v2.as_slice(), &[]);
        assert_eq!(v3.as_slice(), &[]);
        assert_eq!(v4.as_slice(), &[1, 2, 3]);
        assert_eq!(v5.as_slice(), &[1, 2]);
        assert_eq!(v6.as_slice(), &[1, 0, 3]);
        assert_eq!(v7.as_slice(), &[0, 0, 3]);
        assert_eq!(v8.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn add_test() {
        assert_eq!(add(&[1, 2, 3, 4], &[1, 2, 3, 4]), [2, 4, 6, 8]);
        assert_eq!(add(&[1, 2, 3, 4], &[]), [1, 2, 3, 4]);
        assert_eq!(add::<i32>(&[], &[]), []);
        assert_eq!(add(&[1], &[]), [1]);
        assert_eq!(add(&[], &[1]), [1]);
        assert_eq!(add(&[], &[1, 2]), [1, 2]);
        assert_eq!(add(&[1, 2], &[]), [1, 2]);
        assert_eq!(add(&[1, 0, 1, 0], &[0, -1, 0, -1]), [1, -1, 1, -1]);
        assert_eq!(add(&[1, 2, 3, 4], &[1, 2, 3]), [2, 4, 6, 4]);
        assert_eq!(add(&[1, 2, 3], &[1, 2, 3, 4]), [2, 4, 6, 4]);
        assert_eq!(add(&[1, 2, 3], &[1, -2, -3]), [2]);
        assert_eq!(add(&[1, 2, 3], &[-1, -2, -3]), []);
        assert_eq!(
            add(&[10, -5, 100, -50], &[50, 10, -40, -50]),
            [60, 5, 60, -100]
        );
    }

    #[test]
    fn sub_test() {
        assert_eq!(sub(&[1, 2, 3, 4], &[1, 2, 3, 4]), []);
        assert_eq!(sub(&[1, 2, 3, 4], &[]), [1, 2, 3, 4]);
        assert_eq!(sub::<i32>(&[], &[]), []);
        assert_eq!(sub(&[1], &[]), [1]);
        assert_eq!(sub(&[], &[1]), [-1]);
        assert_eq!(sub(&[], &[1, 2]), [-1, -2]);
        assert_eq!(sub(&[1, 2], &[]), [1, 2]);
        assert_eq!(sub(&[1, 0, 1, 0], &[0, -1, 0, -1]), [1, 1, 1, 1]);
        assert_eq!(sub(&[1, 2, 3, 4], &[1, 2, 3]), [0, 0, 0, 4]);
        assert_eq!(sub(&[1, 2, 3], &[1, 2, 3, 4]), [0, 0, 0, -4]);
        assert_eq!(
            sub(&[10, -5, 100, -50], &[50, 10, -40, -50]),
            [-40, -15, 140]
        );
    }

    #[test]
    fn mul_scalar_test() {
        assert_eq!(mul_scalar(&[1, 2, 3, 4], 0), [0, 0, 0, 0]);
        assert_eq!(mul_scalar(&[1, 2, 3, 4], 1), [1, 2, 3, 4]);
        assert_eq!(mul_scalar(&[1, 2, 3, 4], -1), [-1, -2, -3, -4]);
        assert_eq!(mul_scalar(&[1, 2, 3, 4], 10), [10, 20, 30, 40]);
        assert_eq!(mul_scalar(&[1, 2, 3, 4], -10), [-10, -20, -30, -40]);
    }

    #[test]
    fn div_test() {
        assert_eq!(div(&[1], &[1]), [1]);
        assert_eq!(div(&[1, 2], &[1]), [1, 2]);
        assert_eq!(div(&[1], &[1, 2]), []);
        assert_eq!(div(&[-1, 1, 2, 0, 1], &[1, 1, 1]), [2, -1, 1]);
        assert_eq!(div(&[-2, -3, -8, 4], &[-3, 2]), [-3, -1, 2]);
        assert_eq!(div(&[-2, -3, -8, 4], &[-3, 2]), [-3, -1, 2]);
        assert_eq!(div(&[8, 0, -14, 0, 3], &[1, -2, 1]), [-5, 6, 3]);
        assert_eq!(div(&[-4, 19, -24, -1, 10], &[1, -3, 2]), [-4, 7, 5]);
    }

    #[test]
    fn rem_test() {
        assert_eq!(rem(&[1], &[1]), []);
        assert_eq!(rem(&[1, 2], &[1]), []);
        assert_eq!(rem(&[1], &[1, 2]), [1]);
        assert_eq!(rem(&[-1, 1, 2, 0, 1], &[1, 1, 1]), [-3]);
        assert_eq!(rem(&[-2, -3, -8, 4], &[-3, 2]), [-11]);
        assert_eq!(rem(&[8, 0, -14, 0, 3], &[1, -2, 1]), [13, -16]);
        assert_eq!(rem(&[-4, 19, -24, -1, 10], &[1, -3, 2]), []);
    }

    #[test]
    #[should_panic]
    fn div_by_empty_vector_test() {
        div(&[1, 2, 3, 4], &[]);
    }
}
