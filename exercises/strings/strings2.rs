// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


/*
    字符串字面量适用于：
    程序中固定的文本
    不需要修改的字符串
    性能要求高的场景
    &str 适用于：
    函数参数（通用性好）
    字符串处理中的临时视图
    不需要拥有数据所有权的情况
    String 适用于：
    需要修改字符串内容
    运行时构建字符串
    需要拥有数据所有权
 */
fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
