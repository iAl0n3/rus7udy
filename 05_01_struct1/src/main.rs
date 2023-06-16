/* 什么是 Struct
    struct ：结构体
        - 自定义的数据类型‘
        - 为相关联的值命名，打包 => 有意义的组合

    定义 struct
        - 使用 struct 关键字，并为整个 struct 命名
        - 在花括号内，为所有的字段（Field）定义名称和类型

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    实例化 struct
        想要使用 struct，需要创建 struct 的实例：
        - 为每个字段指定具体值
        - 无需按声明的顺序进行指定
  */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("John"),
        email: String::from("example@mail.com"),
        sign_in_count: 1,
        active: true,
    };

    // 取得 struct 里面的某个值：使用点标记法
    user1.email = String::from("example@mail.com");
    // 一旦 struct 的实例是可变的，那么实例中所有的字段都是可变的
}

// struct 作为函数的返回值
fn build_user(email: String, username: String) -> User {
    User {
        // 字段初始化简写
        // email: email,
        // username: username,
        username,
        email,
        sign_in_count: 1,
        active: true,
    }


    /* struct 更新语法
    当你想基于某个 struct 实例来创建一个新实例的时候，可以使用 struct 更新语法
    */
    let user2 = User {
        email: String::from("example@mail.com"),
        username: String::from("John Woke"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 简写
    let user3 = User {
        email: String::from("example@mail.com"),
        username: String::from("John Woke"),
        ..user1
    };

    /* tuple struct
        可定义类似 tuple 的 struct，叫做 tuple struct
        Tuple struct 整体有个名，但里面的元素没有名

        定义 Tuple struct：使用 struct 关键字，后边是名字，以及里面元素的类型
     */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black 和 origin 是不同的类型，是不同 tuple stuct 的实例

    /* Unit-Like Struct
        可定义没有任何字段的 struct，叫做 Unit-Like struct (因为与 ()， 单元类型类似)
        适用于需要在某个类型上实现某个 trait，但是在里面又没有想要存储的数据
     */

    /* Struct 数据的所有权
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        这里的字段使用了 String 而不是 &str；
            - 该 struct 实例拥有其所有的数据
            - 只要 struct 实例是有效的，那么里面的字段数据也是有效的

        struct 里面也可以存放引用，但这需要使用生命周期
     */
}