// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let mut vec0 = vec![22, 44, 66];
        fill_vec(&mut vec0);
        assert_eq!(vec0, [22, 44, 66, 88]);
    }
}
