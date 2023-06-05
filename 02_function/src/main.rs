fn main() {
    println!("Hello, world!");
    first_function();
    second_function(5); // argument
    third_function(5, 5);

    let z = fifth_function(6);
    println!("The value of x is: {}", z);
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

/* 函数的返回值：
    - 在 -> 符号后边声明函数返回值的类型，但是不可以为返回值命名
    - 在 Rust 里面，返回值就是函数体里面最后一个表达式的值

    若想提前返回，需使用 return 关键字，并指定一个值
        - 大多数函数都是默认使用最后一个表达式为返回值
 */
fn fifth_function(z : i32) -> i32 {
    z + 5
}