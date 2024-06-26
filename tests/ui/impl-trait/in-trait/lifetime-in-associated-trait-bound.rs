//@ check-pass

#![feature(associated_type_bounds)]

trait Trait {
    type Type;

    fn method(&self) -> impl Trait<Type: '_>;
}

impl Trait for () {
    type Type = ();

    fn method(&self) -> impl Trait<Type: '_> {
        ()
    }
}

fn main() {}
