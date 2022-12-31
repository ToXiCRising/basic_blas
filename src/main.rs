use crate::level1::{sv_mult, crossproduct, vv_add, dotproduct, norm};
use crate::level2::{sm_mult, mv_mult};
use crate::level3::{mm_mult, frobenius_norm, mm_add};
use crate::helper_functions::{print_vector, print_matrix, fill_matrix, diag_matrix};

mod level1;
mod level2;
mod level3;
mod helper_functions;

fn main() {
    println!("/----------------------------\\");
    println!("|Basic implementation of BLAS|");
    println!("\\----------------------------/\n");
    let a = 2.0;
    let b = vec![1.0, 2.0, 3.0];
    let c = vec![4.0, 3.0, 5.0];
    let d = fill_matrix(4,4, "ones");
    let e = fill_matrix(3,3, "visual");
    let f = fill_matrix(2, 3, "visual");
    let g = fill_matrix(3, 4, "visual");

    let r1 = sv_mult(a.clone(), b.clone());
    let r2 = vv_add(b.clone(), c.clone());
    let r3 = dotproduct(b.clone(), c.clone());
    let r4 = crossproduct(b.clone(), c.clone());
    let r5 = norm(b.clone());

    let r6 = sm_mult(a.clone(), e.clone());
    let r7 = mv_mult(e.clone(), b.clone());

    let r8 = mm_mult(e.clone(), r6.clone());
    let r9 = mm_mult(f.clone(), g.clone());
    let r10 = frobenius_norm(f.clone()); 

    let sys_size = 9;
    let sys_matrix = mm_add(mm_add(diag_matrix(sys_size, -2.0, 0), 
                                                        diag_matrix(sys_size, 1.0, 1)), 
                                                        diag_matrix(sys_size, 1.0, -1));


    println!("Tests for level 1");
    println!("a = {a}");
    print!("b = ");
    print_vector(b);
    print!("c = ");
    print_vector(c);

    print_matrix(sys_matrix);

    print!("Multiplication with scalar a*b: r1 = ");
    print_vector(r1);

    print!("Addition of two vectors b+c: r2 = ");
    print_vector(r2);

    println!("Dotproduct b.c: r3 = {}", r3);

    print!("Crossproduct bxc: r4 = ");
    print_vector(r4);

    println!("Norm of vector b: r5 = {}", r5);

    println!("\nTests for level 2");
    println!("Matrix d = ");
    print_matrix(d);

    println!("Matrix e = ");
    print_matrix(e);

    println!("Matrix e multiplied by scalar a: r6 = ");
    print_matrix(r6);

    println!("Matrix vector multiplication between e and b: r7 = ");
    print_vector(r7);

    println!("\nTests for level 3");
    println!("Matrix multiplication between e and r6: r8 = ");
    print_matrix(r8);

    println!("Matrix f =");
    print_matrix(f);
    println!("Matrix g = ");
    print_matrix(g);
    println!("Matrix multiplication between f and g: r9 = ");
    print_matrix(r9);

    println!("Frobenius norm of matrix f: r10 = {}", r10);
}





