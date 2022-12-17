use crate::helper_functions::{check_core_dimension};

type MatF64 = Vec<Vec<f64>>;

pub fn sm_mult(scalar: f64, mut matrix: MatF64) -> MatF64{
    let rows = matrix.len();
    let columns = matrix[0].len();

    for i in 0..rows{
        for j in 0..columns{
            matrix[i][j] *= scalar;
        }    
    }
    return matrix;
}

pub fn mv_mult(matrix: MatF64, vector: Vec<f64>) -> Vec<f64>{
    let rows = matrix.len();
    let columns = matrix[0].len();
    let l = vector.len();
    check_core_dimension(l, rows);
    let mut result = Vec::new();
    let mut inner: f64;

    for i in 0..rows{
        inner = 0.0;
        for j in 0..columns{
            inner += matrix[i][j]*vector[j];
        }
        result.push(inner);
    }
    return result;
}