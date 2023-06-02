use rand::Rng; //trait
use std::cmp::Ordering;
use std::io; //prelude

fn main() {
    println!("Guessing Game");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("神秘数字是：{}", secret_number);
    
    loop {
        println!("Guess a number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("恭喜你猜对了");
                break;
            }
        }
    }
}
