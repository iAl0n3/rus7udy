use rand::Rng; //trait
use std::io; //prelude

fn main() {
    println!("Guessing Game");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("神秘数字是：{}", secret_number);

    println!("Guess a number");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("无法读取行");
    
    println!("你猜测的数是：{}", guess);
}
