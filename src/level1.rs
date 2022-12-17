use crate::helper_functions::{check_core_dimension};

pub fn sv_mult(scalar: f64, vector: Vec<f64>) -> Vec<f64>{
    let mut result = Vec::new();
    for i in &vector{
        result.push(scalar * i)
    }
    return result;
}

pub fn vv_add(vector1: Vec<f64>, vector2: Vec<f64>) -> Vec<f64>{
    let l = vector1.len();
    let l2 = vector2.len();
    check_core_dimension(l, l2);
    
    let mut result = Vec::new();
    for i in 0..l{
        result.push(vector1[i] + vector2[i]);
    }
    return result;
}

pub fn dotproduct(vector1: Vec<f64>, vector2: Vec<f64>) -> f64{
    let l = vector1.len();
    let l2 = vector2.len();
    check_core_dimension(l,l2);
    let mut result = 0.0;
    for i in 0..l{
        result += vector1[i]*vector2[i];
    }
    return result;
}

pub fn crossproduct(vector1: Vec<f64>, vector2: Vec<f64>) -> Vec<f64>{
    let l = vector1.len();
    let l2 = vector2.len();
    check_core_dimension(l, l2);
    if l != 3{
        panic!("Crossproduct is only defined for vectors of dimension (3)!
        Given vector has dimesion {}", l)
    }
    let mut result = Vec::new();
    for i in 0..l{
        result.push(vector1[(i+1)%3]*vector2[(i+2)%3] - vector1[(i+2)%3]*vector2[(i+1)%3])
    }
    return result;
}

/// Euclidean norm of a vector.
pub fn norm(vector: Vec<f64>) -> f64{
    let mut inner = 0.0;
    for i in &vector{
        inner += f64::powf(*i, 2.0);
    }
    let result = inner.sqrt();
    return result;
}