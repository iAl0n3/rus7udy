/* Struct 方法
    方法和函数类似： fn 关键字、名称、参数、返回值
    方法与函数不同之处：
        - 方法是在 struct （或 enum、trait 对象）的上下文中定义
        - 第一个参数是 self，表示方法被调用的 struct 实例
 */

struct Rectangle {
    width: u32,
    height: u32,
}
/* 定义方法：
    在 impl 块里定义方法
    方法的第一个参数可以是 &self，也可以获得其所有权或可变借用。和其他参数一样

    有更良好的代码组织
 */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法可以有多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let react2 = Rectangle {
        width: 10,
        height: 40,
    };

    let react3 = Rectangle {
        width: 30,
        height: 80,
    };

    let s = Rectangle::square(20);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&react2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&react3));
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

/* 方法调用的运算符：
    Rust 没有 -> 运算符合
    Rust 会自动引用或解引用

    在调用方法是，Rust 根据情况自动添加 &，&mut 或 *，以便 Object 可以匹配方法签名
 */

/* 关联函数
    可以在 impl 块里定义不把 self 作为第一个参数的函数，它们叫做关联函数（不是方法）
    例如： String::from()

    关联函数常用于构造器

    ::符号：
        - 关联函数
        - 模块创建的命名空间

    每个 struct 允许拥有多个 impl 块
 */