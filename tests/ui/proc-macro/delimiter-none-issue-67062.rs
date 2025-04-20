//@ run-pass
//@ proc-macro: delimiter-none-issue-67062.rs

// Regression test for #67062.

#[macro_use]
extern crate delimiter_none_issue_67062;

macro_rules! mbe {
    ($e: expr) => ( add_mul!($e) )
}

fn main() {
    // Order of operations check. This should evalue to (2 + 2) * 2, but using
    // Delimiter::None instead of parentheses.
    let x = mbe!(2 + 2);
    assert_eq!(x, 8);
}
