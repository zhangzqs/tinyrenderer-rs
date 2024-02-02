use crate::vec::Vector;
use num_traits::Num;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T: Num, const R: usize, const C: usize> {
    data: [Vector<T, C>; R],
}

impl<T: Num + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [Vector<T, C>; R]) -> Self {
        Self { data }
    }

    pub fn new_zero() -> Self {
        Self {
            data: [Vector::new_zero(); R],
        }
    }

    pub fn get(&self, rowi: usize, coli: usize) -> T {
        self.data[rowi][coli]
    }

    pub fn set(&mut self, rowi: usize, coli: usize, val: T) {
        self.data[rowi][coli] = val
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut ret = Matrix::<T, C, R>::new_zero();
        for r in 0..R {
            for c in 0..C {
                ret.set(c, r, self.get(r, c));
            }
        }
        ret
    }
}

impl<T: Num + Copy, const N: usize> Matrix<T, N, N> {
    pub fn identity() -> Self {
        let mut ret = Self::new_zero();
        for i in 0..N {
            ret.set(i, i, T::one());
        }
        ret
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> ops::Add for Matrix<T, R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> ops::Sub for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> ops::Index<usize> for Matrix<T, R, C> {
    type Output = Vector<T, C>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// 矩阵数乘运算
impl<T: Num + Copy, const R: usize, const C: usize> ops::Mul<T> for Matrix<T, R, C> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for i in 0..R {
            data[i] = data[i] * rhs;
        }
        Matrix::new(data)
    }
}

/// R行C列的矩阵*长度为C的列向量=长度为R的列向量
impl<T: Num + Copy, const R: usize, const C: usize> ops::Mul<Vector<T, C>> for Matrix<T, R, C> {
    type Output = Vector<T, R>;

    fn mul(self, rhs: Vector<T, C>) -> Self::Output {
        let mut data = [T::zero(); R];
        for i in 0..R {
            data[i] = self.data[i] * rhs;
        }
        Vector::new(data)
    }
}

/// R行C列的矩阵*C行D列的矩阵=R行D列的矩阵
impl<T: Num + Copy, const R: usize, const C: usize, const D: usize> ops::Mul<Matrix<T, C, D>>
    for Matrix<T, R, C>
{
    type Output = Matrix<T, R, D>;

    fn mul(self, rhs: Matrix<T, C, D>) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            for j in 0..D {
                for k in 0..C {
                    data[i][j] = data[i][j] + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        Matrix::new(data)
    }
}
