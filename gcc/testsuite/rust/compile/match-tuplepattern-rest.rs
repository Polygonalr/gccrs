fn main() {
    let x = (1, 4, 2, 3);

    match x {
        (1, .., 3) => {},
        _ => {}
    }
}