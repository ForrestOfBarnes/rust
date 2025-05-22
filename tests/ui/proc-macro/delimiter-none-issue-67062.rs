//@ run-pass
//@ proc-macro: delimiter-none-issue-67062.rs

// Regression test for #67062.

#[macro_use]
extern crate delimiter_none_issue_67062;

macro_rules! mul_clone {
    ($expr:expr) => { clone_identity!($expr * $expr) };
}

macro_rules! mul_extend {
    ($expr:expr) => { extend_identity!($expr * $expr) };
}

fn main() {
    // This example comes from issue #74036, which was determined to be a
    // duplicate of #67062. Both macros below should expand to (1 + 1) * (1 + 1)
    let x = mul_clone!(1 + 1);
    let y = mul_extend!(1 + 1);
    assert_eq!(x, 4);
    assert_eq!(y, 4);
}
