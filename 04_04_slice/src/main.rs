/* 切片
        Rust 的另外一种不持有所有权的数据类型：切片（slice）
*/

/* 一道题目：
    - 编写一个函数
    - 返回它在这个字符串里找到的第一个单词
    - 如果函数没找到任何空格，那么整个字符串就被返回
 */

fn main() {
    let mut s = String::from("hello world");
    // let word_index = first_world(&s);
    let word_index = second_world(&s);
    // s.clear();

    println!("word_index = {}", word_index);
}

/* 字符串切片
    字符串切片是指向字符串中一部分内容的引用
    形式：[开始索引..结束索引]
    范围：从开始索引到结束索引之间的所有元素
    Notes:
        - 字符串切片的范围索引必须是发生在有效的 UTF-8 字符边界内
        - 如果尝试从一个多字节的字符串中创建字符串切片，程序会报错并退出
 */
fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 使用字符串切片重写
fn second_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/* 字符串字面值是切片
    字符串字面值被直接存储在二进制程序中
    let s = "Hello World";
    变量 s 的类型是 &str，它是一个指向二进制程序特定位置的切片
        - &str 是不可变引用，所以字符串字面值也是不可变的
 */

/* 将字符串切片作为参数传递
    fn second_world(s: &String) -> &str
    有经验的 Rust 开发者会采用 &str 作为参数类型，这样就可以同时接收 String 和 &str 类型的参数了：
    fn second_world(s: &str) -> &str
        - 使用字符串切片，直接调用该函数
        - 使用 String，可以创建一个完整的 String 切片来调用该函数
    定义函数时使用字符串切片来代替字符串引用会使我们的 API 更加通用，且不会损失任何功能
 */
