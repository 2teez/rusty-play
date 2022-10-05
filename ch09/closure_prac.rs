#![allow(dead_code, unused)]

use std::cmp::Ordering;

fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort();
    println!("{:?}", arr);

    arr.sort_by(|a, b| {
        if a < b {
            return Ordering::Greater;
        } else if a > b {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    });
    println!("{:?}", arr);
}
