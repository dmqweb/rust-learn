fn main() {
    println!("今天是:{}", if_sun(&true));
    print!("我的宠物是:{}", animal_habitat("snake"));
    match_fn();
}
// if--else分支
fn if_sun(flag: &bool) -> &str {
    if *flag {
        "大晴天"
    } else {
        "下雨天"
    }
}
fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crab" {
        "Beach"
    } else if animal == "gopher" {
        "Burrow"
    } else if animal == "snake" {
        "Desert"
    } else {
        "Unknown"
    };
    identifier
}
// match分支
fn match_fn() {
    let x: Option<i32> = Some(10);
    let y: Result<i32, &str> = Ok(20);
    match x {
        Some(value) => println!("x有值{}", value),
        None => println!("x没有值"),
    }
    match y {
        Ok(value) => println!("y有值{}", value),
        Err(e) => println!("y报错{}", e),
    }
}
