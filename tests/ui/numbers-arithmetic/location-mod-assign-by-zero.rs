//@ run-fail
//@ ignore-wasm32
//@ error-pattern:location-mod-assign-by-zero.rs

fn main() {
    let mut a = 1;
    a %= &0;
}
