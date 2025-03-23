fn main() {
    let x = 2;

    match x { // compiles but output gimple is incorrect
        a @ 2 => {},
        _ => {}
    }
}

