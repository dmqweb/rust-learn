trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}
struct SomeSoftware;
struct OtherSoftware;
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}
// &dyn是对trait对象的引用
fn compare_license_types(software1: &dyn Licensed, software2: &dyn Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
fn main() {
    println!("{}", compare_license_types(&SomeSoftware, &OtherSoftware));
    println!("{}", compare_license_types(&OtherSoftware, &SomeSoftware));
}
