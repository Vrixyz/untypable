use untypable::returns_leaked_type;

pub fn main() {
    // We can't use its type, but we can use `_`. Inference works.
    let variable: _ = returns_leaked_type();
    dbg!(variable).how_do_i_discover_this();
}
