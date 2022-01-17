use std::{fs::File, io::{Read, self}};
/*
unwrap: if we sure that no error will occur
match: Customize / handle it manually
? operator states "if the result is Ok, carry on in this function. Else if it is an Err, propagate it back up the stack to the function that called me."
*/
fn read_stuff_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_stuff_from_file_alt() -> Result<String, io::Error> {
    let mut f = File::open("hello1.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
fn main() {
    let f1 = read_stuff_from_file();
    let f2 =read_stuff_from_file_alt();

    println!("Contents f1: {:?}",f1);
    println!("Contents f2: {:?}",f2);

}   