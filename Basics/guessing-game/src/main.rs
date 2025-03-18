use std::io;
fn main() {
    println!("Enter a number :");
    let mut g = String::new();
    io::stdin().read_line(&mut g).expect("Input Failed !!!");
    println!("You Guessed {}",g);
    
}
