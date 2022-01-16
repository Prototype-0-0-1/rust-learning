fn main() {
    let ports: u32 = 65535;
    println!("val of ports: {}",ports);
    /*
    There are 2 types of strings in Rust: String and &str
    String is a growable allocated data structure whereas str is an immutable fixed-length string somewhere in memory.
    &str is a string slice of string.

    https://doc.rust-lang.org/book/ch08-02-strings.html
    */

    let x: i8 = -1;
    println!("Val of x: {}",x);
}
