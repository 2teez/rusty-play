
fn main() {
    let is_even = |x| 0 == x % 2;

    for num in 0 ..= 10 {
        println!("{:02} is {}", num, if is_even(num) {
            "even number"
        } else {
            "odd number"
        });
    }
}
