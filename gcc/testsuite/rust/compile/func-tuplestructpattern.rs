struct Struct(i32, i32);

fn struct_pattern(Struct(a, b): Struct) -> i32 {
    a + b
}

fn main() {
    struct_pattern(Struct(2, 3));
}