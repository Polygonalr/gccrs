fn tuple_deconstruct((a, b, ..) :(i32, i32, i32)) -> i32 {
    a + b
}

fn main() {
    tuple_deconstruct((1, 2, 3));
}