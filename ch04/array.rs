fn main() {
    let mut arr = [3, 6, 7, 2, 1];
    arr.sort();
    println!("{:?}", arr);

    for v in arr {
        print!("{}", v);
    }
    // using a while loop
    let mut counter = 0;
    while counter < arr.len() {
        print!("{}", arr[counter]);
        counter += 1;
    }

    for v in 0..24 {
        if is_prime(v) {
            println!("It is a prime number: {:?}", v);
        }
    }
    // summing up first 10 prime numbers
    let mut first_ten_prime_nums = vec![];
    const COUNT: usize = 10;
    let mut num = 0;
    loop {
        if is_prime(num) {
            first_ten_prime_nums.push(num);
            if COUNT == first_ten_prime_nums.len() {
                break;
            }
        }
        num += 1;
    }
    println!(
        "{:?}, {}, ",
        first_ten_prime_nums,
        first_ten_prime_nums.iter().sum::<i32>(),
        //first_ten_prime_nums.iter().product::<i32>()
    );
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    for v in 2..(num - 1) {
        if num % v == 0 {
            return false;
        }
    }
    true
}
