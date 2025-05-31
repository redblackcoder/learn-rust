use std::fmt::*;

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


// The following struct is for the activity.
#[derive(Debug)]
#[allow(dead_code)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({0:^6.2} {1:^6.2})\n({2:^6.2} {3:^6.2})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    // Transpose the matrix by swapping the elements.
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug:\n{:?}", matrix);
    println!("Display:\n{}", matrix);
    println!("Transposed matrix:\n{}", transpose(matrix));

    let complex = Complex { real: 3.0, imag: 4.0 };
    println!("{:?}", complex);
    println!("{}", complex);
}
