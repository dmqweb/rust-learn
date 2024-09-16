fn main() {
    //其中: \符号代表不会读取该行
    let penguin_data = "\
    \
    common name,length (cm)
    Little penguin,33
    \
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        // 如果是第一行,获取为空就continue
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprint!("debug:{:?}->{:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}
