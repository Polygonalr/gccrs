enum MyOption {
    Some((i32, i32)),
    None,
}

pub fn toto(i : MyOption) -> i32 {
    if let MyOption::Some((a, b)) = i {
        a + b
    } else {
        23i32
    }
}
