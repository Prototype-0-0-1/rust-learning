fn main() {
    const HUNDRED_THOUSAND: u32 = 100_000;

    /*
    We can use the _ character to denote a space in number without it affecting the value itself. This is purely for readability.
    */
    
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x + 1 is: {}", x);
    /*
    
    */
    let y = HUNDRED_THOUSAND + 1;
    println!("Hundred Thousand, a const val, is: {}",HUNDRED_THOUSAND);
    println!("y is const val + 1 : {}",y);

    let word = "hello";
    println!("val of word before manipulation: {}",word);
    let word = word.len();
    println!("val of word after manipulation: {}",word);

    /*
    let mut word = "hello";
    word = word.len();
        
        this would give: mismatched types
            expected `&str`, found `usize`rustcE0308
    mut can not change types
    */
    const SAMPLE_WORD: &str = "yes";
    /*
    to create a string as a constant, need the 
        : &str
    to be in the definition
    */
    println!("Constant string: {}",SAMPLE_WORD);

}