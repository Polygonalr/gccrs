struct Struct(i32);

fn struct_pattern(Struct { 0: a }: Struct) -> i32 {
    a
}

fn main() {
    struct_pattern(Struct(2));
}