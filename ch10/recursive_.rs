#![allow(dead_code, unused)]

/*use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;
*/

fn main() {
    // This is a very large number.
    //println!("fib(1000) = {}", fib(1000));

    println!("Using a tail factorial: {}", tail_fac(5));

    for num in 1..1_00_000 {
        println!(" {} -> {}", num, fac(num));
    }
}

fn fac(num: u128) -> u128 {
    if num == 1 {
        return 1;
    }
    num * fac(num - 1)
}

fn tail_fac(num: u32) -> u32 {
    fn fact(num: u32, total: u32) -> u32 {
        if num == 1 {
            return total;
        }
        fact(num - 1, num * total)
    }
    fact(num, 1)
}

// Calculate large fibonacci numbers.
/*fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}*/
