#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });
    match &optional_point { //这里要使用&借用，直接使用变量会导致变量被移入到分支中，使得后续无法正常使用
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }
    println!("{optional_point:?}");
}
