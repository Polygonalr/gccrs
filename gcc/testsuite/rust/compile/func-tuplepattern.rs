fn tuple_deconstruct((a, b, c) :(i32, i32, i32)) -> i32 {
    a + b + c
}

fn main() {
    tuple_deconstruct((1, 2, 3));
}