fn main() {
    let pi = 3.14;
    let radius: f32 = 5.0;
    let area = pi * radius.powi(2);
    println!("圆的面积是：{radius:.2} is {area:.5}");
    let mut res = 42;

    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{res}");
}
