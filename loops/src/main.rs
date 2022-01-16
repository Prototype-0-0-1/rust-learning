fn main() {
    
    loop{
        println!("Hello, world!");
        break;
    }

    let mut number = 3;
    while number>0 {
        println!("{} is the iter value",number);
        number -= 1
    }

    let a = [10,20,30,40,50];
    // Cool, arrays

    println!("Using .iter(), it uses &i32");
    for each_val in a.iter(){
        println!("The value is {}",each_val);
    }

    println!("Without using .iter(), it uses i32");
    for each_val in a{
        println!("The value is {}",each_val);
    }

}
