//@ revisions: classic next
//@[next] compile-flags: -Znext-solver
//@[next] check-pass

fn ice()
where
    for<'w> fn(&'w ()): Fn(&'w ()),
{
}

fn main() {
    ice();
    //[classic]~^ ERROR expected a `Fn(&'w ())` closure, found `fn(&'w ())`
}
