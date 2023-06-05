fn main() {
/* 控制流：
    if 表达式
    - if 表达式允许你根据条件来执行不同的代码分支，这个条件必须是 bool 类型
    if 表达式中，与条件相关联的代码块就叫做分支 （arm)
    - 可选的，在后面可以加上一个 else 表达式
 */
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
/* 使用 else if 来处理多重条件：
    但如果使用了多于一个 else if ，那么最好使用 match 来重构代码
 */

    let  number2 = 6;
    match number2 % 4 {
        0 => println!("number is divisible by 4"),
        _ => match number2 % 3 {
            0 => println!("number is divisible by 3"),
            _ => match number2 % 2 {
                0 => println!("number is divisible by 2"),
                _ => println!("number is not divisible by 2 or 3"),
            }
        }
    }

/* 在 let 语句中使用 if
    因为 if 是一个表达式，所以可以将它放在 let 语句中等号的右边
 */
    let condition = true;
    let number3 = if condition { 5 } else { 6 };
    println!("The value of number3 is: {}", number3);

/* 循环
    
    loop 循环
    loop 关键字告诉 rust 反复执行一块代码，直到你喊停

    loop {
        println!("again!");
    }

    可以在 loop 循环中使用 break 关键字来告诉程序何时停止循环
 */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

/* while 条件循环
    另外一种常见下循环模式是每次执行循环体之前都判断一次条件
 */
    let mut number4 = 3;

    while number4 != 0 {
        println!("{}!", number4);
        number4 -= 1;
    }

    println!("LIFTOFF!!!");

/* for 循环
    可以使用 while 或者 loop 来遍历集合，但是易错且低效
    使用 for 循环更加简洁紧凑，它可以针对集合中的每个元素来执行一些代码
 */
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
// 由于 for 循环的安全、简洁性，所以它在 Rust 里用的最多

/* Range
    - 是由标准库提供的
    - 指定一个开始数字和一个结束数字，Range 可以生成它们之间的数字（不含结束）
    rev 方法可以反转 Range
 */
    for number5 in (1..4).rev() {
        println!("{}!", number5);
    }
    println!("LIFTOFF!!!");
}
