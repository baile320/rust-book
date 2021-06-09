fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);

    println!("The number 0b111_0000 is: {}", 0b111_0000);

    let blah: u8 = b'A';
    println!("The value of blah is: {}", blah);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of tup[1] is: {}", tup.1);
}
