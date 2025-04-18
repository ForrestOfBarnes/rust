//! Check that we do not allow assigning twice to an immutable variable. This test also checks a
//! few pieces of borrowck diagnostics:
//!
//! - A multipart borrowck diagnostics that points out the first assignment to an immutable
//!   variable, alongside violating assignments that follow subsequently.
//! - A suggestion diagnostics to make the immutable binding mutable.

//@ run-rustfix

fn main() {
    let v: isize;
    //~^ HELP consider making this binding mutable
    //~| SUGGESTION mut
    v = 1;
    //~^ NOTE first assignment
    println!("v={}", v);
    v = 2;
    //~^ ERROR cannot assign twice to immutable variable
    //~| NOTE cannot assign twice to immutable
    println!("v={}", v);
}
