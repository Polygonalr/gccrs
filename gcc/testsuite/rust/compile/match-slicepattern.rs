fn main() {
    let a = [0, 0, 0];

    match a { // does not compile
        [0, _, _] => {}
        _ => {}
    }
}
