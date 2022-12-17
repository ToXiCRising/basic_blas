
fn check_core_dimension(vector1: Vec<f64>, vector2: Vec<f64>){
    if vector1.len() != vector2.len(){
        panic!("Core dimensions of given vectors are not the same!
        Dimension of vector 1: ({})
        Dimension of vector 2: ({})", vector1.len(), vector2.len())
    }
}

pub fn sv_mult(scalar: f64, vector: Vec<f64>) -> Vec<f64>{
    let mut result = Vec::new();
    for i in &vector{
        result.push(scalar * i)
    }
    result
}

pub fn vv_add(vector1: Vec<f64>, vector2: Vec<f64>) -> Vec<f64>{
    check_core_dimension(vector1.clone(), vector2.clone());
    let l = vector1.len();
    let mut result = Vec::new();
    for i in 0..l{
        result.push(vector1[i] + vector2[i]);
    }
    result
}

pub fn dotproduct(vector1: Vec<f64>, vector2: Vec<f64>) -> f64{
    check_core_dimension(vector1.clone(), vector2.clone());
    let l = vector1.len();
    let mut result = 0.0;
    for i in 0..l{
        result += vector1[i]*vector2[i];
    }
    result
}

pub fn crossproduct(vector1: Vec<f64>, vector2: Vec<f64>) -> Vec<f64>{
    check_core_dimension(vector1.clone(), vector2.clone());
    let l = vector1.len();
    if l != 3{
        panic!("Crossproduct is only defined for vectors of dimension (3)!
        Given vector has dimesion {}", l)
    }
    let mut result = Vec::new();
    for i in 0..l{
        result.push(vector1[(i+1)%3]*vector2[(i+2)%3] - vector1[(i+2)%3]*vector2[(i+1)%3])
    }
    result
}

pub fn norm(vector: Vec<f64>) -> f64{
    let mut inner = 0.0;
    for i in &vector{
        inner += f64::powf(*i, 2.0);
    }
    let result = inner.sqrt();
    result
}