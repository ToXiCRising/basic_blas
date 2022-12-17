type MatF64 = Vec<Vec<f64>>;

//------Creation of matricies------//

/**Creates Matrix with n rows by m columns.
 * Fill options: zeros (default), ones, visual
 */
pub fn fill_matrix(rows: usize, columns: usize, fill: &str) -> MatF64{

    let mut matrix = vec![vec![0.0; columns]; rows];

    if fill == "visual"{
        for i in 0..rows{
            for j in 0..columns{
                matrix[i][j] = i as f64*columns as f64 +j as f64;
            }
        }
    }
    else if fill == "ones"{
        matrix = vec![vec![1.0; columns]; rows];
    }
    return matrix;
}

//------Output of LA-objects------//
pub fn print_vector(vector: Vec<f64>){
    let l = vector.len();
    print!("v[");
    for i in 0..l{
        if i < l-1{
            print!("{} ", vector[i]);
        }
        else{
            print!("{}", vector[i]);
        }
    }
    println!("]")
}

pub fn print_matrix(matrix: MatF64){
    let rows = matrix.len();
    let columns = matrix[0].len();

    print!("M");
    for i in 0..rows{
        if i == 0{ 
            print!("[");
        }
        else{
            print!(" [");
        }
        for j in 0..columns{
            if j < columns-1{
                print!("{} ",matrix[i][j]);
            }
            else{
                print!("{}",matrix[i][j]);
            }
        }
        print!("]");
        println!();
    }
}

//------testing functions------//
pub fn check_core_dimension(dim1: usize, dim2: usize){
    if dim1 != dim2{
        panic!("Core dimensions of given vectors are not the same!
        Dimension of vector 1: ({})
        Dimension of vector 2: ({})", dim1, dim2)
    }
}