# Multiple Mutable References Error in Rust

This example demonstrates a common error in Rust when dealing with mutable references.  Rust's borrow checker prevents data races by disallowing multiple mutable references to the same variable.  The code will not compile due to this restriction. The solution shows how to correctly use borrowing to avoid this error.

## Bug
The `bug.rs` file contains code that attempts to create two mutable references (`y` and `z`) to the same variable `x`. This violates Rust's borrowing rules.

## Solution
The `bugSolution.rs` file presents a solution that avoids the error by using a different approach (e.g. using only one mutable reference, changing the ownership and other methods if required).