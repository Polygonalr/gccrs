fn main() {
    struct A { x: i32, y: i32 }
    let a = A { x: 0, y: 0 };

    match a {
        A { x: 0, y: 1} => {},
        _ => {}
    }
}
