fn main() {
    let var1 =  6;
    if var1 == 6{
        println!("oh no the var1 is 6");
    }
    else {
        println!("yay the var1 isn't 6");
    }

    fn func() -> i8{
        9
    }

    let var2 = 6 + func();

    let result = if var2 == 6 {15} else {200};
    let _output =
    if var2 == 15 {
        println!("Var2 is 15");
        9;
    }else {
        println!("Var2 is not 15");
        10;
    };

    println!("Final value of result variable = {}",result);

}
