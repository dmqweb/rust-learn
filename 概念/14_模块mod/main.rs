use std::time::{SystemTime, UNIX_EPOCH}; //使用系统时间
#[allow(dead_code)]
// 使用mod关键字创建模块
mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
    //模块中的函数默认是私有的，可以在模块内任意访问，如果要被外部访问要使用关键字pub
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}
#[allow(dead_code)]
mod delicious_snacks {
    //模块中的模块默认是私有的，要在外部访问必须要使用use self并暴露出去
    pub use self::fruits::APPLE as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }
    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}
fn main() {
    sausage_factory::make_sausage();
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
