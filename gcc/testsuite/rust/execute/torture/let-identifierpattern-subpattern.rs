enum Foo {
    Bar(i32)
}

fn main() -> i32 {
    let foo @ Foo::Bar(bar) = Foo::Bar(0);
    let mut ret = 1;

    match foo {
        Foo::Bar(n) => { ret = bar }
    }

    ret
}