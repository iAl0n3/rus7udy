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

    // 所有权的转移
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // 声明一个变量，但是不会获得所有权
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    print!("The length of '{}' is {}.", s5, len);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

/*  返回值与作用域：
    - 函数在返回值的过程中同样也会发生所有权的转移
    一个变量的所有权总是遵循同意的模式
        - 把一个值赋给其他变量时就会发生移动
        - 当一个包含 heap 数据的变量离开作用域时，它的值就会被 drop 函数清理，除非数据的所有权被转移到另一个变量上了
 */
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/*  如何让函数使用某个值，但不获得其所有权
    Rust 有一个特性叫做 “引用(Reference)”
 */
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}