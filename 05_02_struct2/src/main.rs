// struct 例子：计算长方形的面积

// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

fn main() {
    // let w = 30;
    // let l = 50;
    //
    // println!("area = {}", area(w, l));
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    println!("area = {}", area(&rect));

    println!("{:#?}", rect);
}