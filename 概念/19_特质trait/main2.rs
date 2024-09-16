#![allow(dead_code)]
trait Licensed {
    fn licensing_info(&self) -> String {
        "你好世界".to_string()
    }
}
struct SomeSoftware {
    version_number: i32,
}
struct OtherSoftware {
    version_number: String,
}
impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.
fn main() {
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware {
        version_number: "v2.0.0".to_string(),
    };
    println!("{}", some_software.licensing_info());
    println!("{}", other_software.licensing_info());
}
