#[allow(dead_code)] // disable `dead_code` which warn against unused module
struct Structure(i32);

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("Hello, world!");
    println!("I am a Rustacean!");

    println!("This struct `{}` will print now!", Structure(3));

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!("Hello {0} is {1:.5}", "x", 0.01);
    format!("{:#?}", (100, 200));

    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

    let point = Point { x: 3.0, y: 4.0 };
    println!("{:?}", point); // Debug formatting
    println!("{:#?}", point); // Pretty Debug formatting
}
