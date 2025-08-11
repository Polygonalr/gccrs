fn foo(a: &[u32]) {
    // { dg-warning "function is never used: .foo." "" { target *-*-* } .-1 }
    match a {
        [first, ..] => {}
        [.., last] => {}
        _ => {}
    }
}
