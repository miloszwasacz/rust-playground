use std::fmt::Display;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let Matrix(f0, f1, f2, f3) = m;
    Matrix(f0, f2, f1, f3)
}

fn main() {
    /*
    let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
    */
    let m = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{m:?}");
    println!("{m}");
    println!("{}", transpose(m));
}
