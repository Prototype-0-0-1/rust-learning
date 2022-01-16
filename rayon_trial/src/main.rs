use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    // iter was changed to par_iter
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}

fn main() {

    let inp_array = [2;32];
    
    println!("Input array is {:?}",inp_array);
    let sum_square = sum_of_squares(&inp_array);

    println!("Squared sum = {}",sum_square);

}
