use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    println!("\nIterating over an array by index:");
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    println!("\nIterating over an array by index, and panicing on out-of-bounds access:");
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        let xval = xs.get(i).expect(&format!("Array index {} out of bounds.", i));
        println!("{}: {}", i, xval);
    }
}