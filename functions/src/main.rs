fn hello() -> u16{
    println!("hello!");
    6
}


/*
fn hello(){
    8172192: u16;
}

fn return(){
    6;
}
 */
fn test(name: &str) {
    println!("{}", name);
    fn print_name(name: String) -> u16{
        println!("The name from inner function is {}", name);
        5
    }
    let new_name: String= name.to_string();
    // Using String::from because the function's input is a String and not a &str
    print_name(new_name);
}


fn main() {
    
    fn main(){
        println!("This is also main");
    }
    println!("I do not return!");
    let x = hello();
    println!("This is x: {}",x);
    main();
    let s_name = String::from("SampleName");
    test(&s_name);
    /*
    String (sample_str) -> &str => just use &sample_str
    Using & before the variable name of the string variable name will make it a &str
    */
    //print_name(s_name);

}
