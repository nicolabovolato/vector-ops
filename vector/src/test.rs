#[cfg(test)]
mod test {
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
