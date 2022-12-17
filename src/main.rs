use crate::level1::{sv_mult, crossproduct, vv_add, dotproduct, norm};

mod level1;
mod level2;
mod level3;

fn main() {
    println!("Basic implementation of BLAS");
    let a = 2.0;
    let b = vec![1.0,2.0,3.0];
    let c = vec![4.0, 3.0, 5.0];
    let r1 = sv_mult(a.clone(), b.clone());
    let r2 = vv_add(b.clone(), c.clone());
    let r3 = dotproduct(b.clone(), c.clone());
    let r4 = crossproduct(b.clone(), c.clone());
    let r5 = norm(b.clone());

    println!("a = {a}");
    print!("b = ");
    print_vector(b);
    print!("c = ");
    print_vector(c);

    print!("Multiplication with scalar a*b: r1 = ");
    print_vector(r1);

    print!("Addition of two vectors b+c: r2 = ");
    print_vector(r2);

    println!("Dotproduct b.c: r3 = {}", r3);

    print!("Crossproduct bxc: r4 = ");
    print_vector(r4);

    println!("Norm of vector b: r5 = {}", r5);
}

fn print_vector(vector: Vec<f64>){
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
