fn main() {
    let mut res = 42;
    let option = Some(12);
    // 使用 if let 替代 for 循环来处理 Option
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
