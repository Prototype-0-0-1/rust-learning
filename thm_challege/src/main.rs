extern crate base64;
extern crate rot13;

use base64::{encode, decode};
use rot13::rot13;


fn main() {
    println!("ROT 13 of Hello World! = {}", rot13::rot13("Hello World!"));
    let a = b"Hello World";

    let b= encode(a);
    println!("Base 64 of Hello World: {}",b);
    
    let c= decode(b);
    println!("{:?}",c.unwrap());
    
    let base_input = String::from("M3I6r2IbMzq9");
    
    let stage_1_rot13 = rot13::rot13(&base_input);
    println!("Step 1: ROT13 \t\t\t: {}",stage_1_rot13);
    
    let stage_2_base64d_bytes = decode(stage_1_rot13).unwrap();
    println!("Step 2: Base64 -d (As bytes) \t: {:?}",stage_2_base64d_bytes);
    let stage_2_base64d_str = std::str::from_utf8(&stage_2_base64d_bytes).unwrap();
    println!("Step 2: Base64 -d (As string) \t: {}",stage_2_base64d_str);

    let stage_3_rot13 = rot13::rot13(stage_2_base64d_str);
    println!("Step 3: ROT13 \t\t\t: {}",stage_3_rot13);
    
}