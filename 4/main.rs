fn main() {
    let tup = (500, 6.4, 1);
    let a: (i32, char, f64) = (500, 'a', 1.1);
    let (x, y, z) = tup;
    println!("{}", x);
}

