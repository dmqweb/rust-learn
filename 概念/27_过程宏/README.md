```rust
/**
 * 库代码中：
 */
//先在Cargo.toml中创建一个新的crate，设置类型proc-macro=true
// [lib]
// proc-macro = true
//然后在库代码中定义过程宏
use proc_macro::TokenStream;
#[proc_macro]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let output = "println!(\"Hello, world!\");";
    output.parse().unwrap()
}
/**
 * 主项目中：
 */
//在主项目中添加宏库依赖
// [dependencies]
// my_macro_lib = { path = "../my_macro_lib" }
//使用宏库：
use my_macro_lib::hello_world;
fn main() {
    hello_world!();
}

```