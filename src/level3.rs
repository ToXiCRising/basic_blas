use crate::helper_functions::{check_core_dimension};

type MatF64 = Vec<Vec<f64>>;

pub fn mm_mult(matrix1: MatF64, matrix2: MatF64) -> MatF64{
    let r1 = matrix1.len();
    let c1 = matrix1[0].len();
    let r2 = matrix2.len();
    let c2 = matrix2[0].len();
    check_core_dimension(c1, r2);
    let mut result = vec![vec![0.0; c2]; r1];
    let mut inner: f64;
    for i in 0..r1{
        for j in 0..c2{
            inner = 0.0;
            for k in 0..r2{
                inner += matrix1[i][k]*matrix2[k][j];
            }
            result[i][j] = inner;
        }
    }
    return result;
}

pub fn frobenius_norm(matrix: MatF64) -> f64{
    let rows = matrix.len();
    let columns = matrix[0].len();
    let mut _inner = 0.0;

    for i in 0..rows{
        for j in 0..columns{
            _inner += f64::powf(matrix[i][j], 2.0);
        }
    }
    let result = _inner.sqrt();
    return result;
}