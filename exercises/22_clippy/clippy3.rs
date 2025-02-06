// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 避免在 is_none() 后调用 unwrap()
    if let Some(x) = my_option {
        println!("{:?}", x);
    }

    let my_arr = &[
        -1, -2, -3,  // 添加逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // 使用 Vec::new() 创建空向量
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // 使用 std::mem::swap
    println!("value a: {value_a}; value b: {value_b}");
}
