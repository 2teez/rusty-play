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

    // using a shorter form
    arr.sort_by(|a, b| a.cmp(b));
    println!("{:?}", arr);

    arr.sort_by(|a, b| (-*a).cmp(&-*b));
    println!("{:?}", arr);

    let summation = |vec| -> i32 {
        let mut sum = 0;
        for val in vec {
            sum += val;
        }
        sum
    };

    println!(
        "{:?}, {:?}",
        summation(vec![2, 8, 0, 3, 7, 5]),
        summation([6, 9, 7, 21].to_vec())
    );

    println!(
        "{:?}",
        (|v: &mut Vec<i32>, factor: u32| {
            for v in v.iter_mut() {
                *v *= factor as i32;
            }
            v.to_vec()
        })(&mut vec![3, 6, 9], 5)
    );
}
