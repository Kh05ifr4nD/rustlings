trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为字符串向量实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut v = self;
        v.push(String::from("Bar"));
        v
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
