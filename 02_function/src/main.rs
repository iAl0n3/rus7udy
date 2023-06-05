fn main() {
    println!("Hello, world!");
    first_function();
    second_function(5); // argument
    third_function(5, 5);
}
/* 函数：
    - 声明函数使用 fn 关键字
    - 依照惯例，针对函数和变量名，Rust 使用 snake case 命名规范：
        + 所有的字母都是小写的，单词之间使用下划线分开
 */
fn first_function() {
    println!("Another function.");
}

/* 函数的参数：
    - paramenters, arguments
    - 在函数的签名里，必须声明每个参数的类型
 */
fn second_function(x: i32) {    // parameter
    println!("The value of x is: {}", x);
}

fn third_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/* 函数体中的语句与表达式：
    - 函数体是由一系列语句组成，可选的由一个表达式结束
    - Rust 是一个基于表达式的语言
    - 语句是执行一些动作的指令
    - 表达式会计算产生一个值
 */
fn forth_function() {
    let x = 5;
}