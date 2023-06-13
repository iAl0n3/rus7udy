fn main() {
    /* 所有权和函数：
        在语义上，将值传递给函数和把值赋给变量是类似的
            - 将值传递给函数将会发生移动或复制
     */
    let s = String::from("hello");

    take_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("x: {}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

/*  返回值与作用域：
    函数在返回值的过程中同样也会发生所有权的转移

 */
