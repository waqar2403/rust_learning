use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let sec = rand::thread_rng().gen_range(1..=100);

loop{
    println!("Guess a number (1-100) :");

    let mut g = String::new();
    io::stdin().read_line(&mut g).expect("Input Failed !!!");

    let g : u32 = match g.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    
    println!("You Guessed {}",g);
    
    match g.cmp(&sec){
        Ordering::Less => println!{"To Low :("},
        Ordering::Greater => println!{"To High :("},
        Ordering::Equal => {println!("Is Equal :)");break;},
    }
    } 
}
