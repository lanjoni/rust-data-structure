fn main() {
    /*
        By default variables in Rust are not mutable, so this would be the implementation 
        of an array of integers in Rust with the variable "numbers" being immutable!
    */
    
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("The first number is: {}", numbers[0]);

    /*
        We have practically the same implementation as the one shown above, but this time 
        defining the variable as mutable!
    */

    let mut new_numbers: [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    println!("The first number is: {}", new_numbers[0]);

    new_numbers[0] = 11;

    println!("Now, the first number is: {}", new_numbers[0]);
}
