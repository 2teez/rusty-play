fn main() {
    // if statement
    let n = 4;

    if n < 0 {
        print!("Number is negative.");
    } else {
        println!("Number is positive.");
    }

    // if expression
    println!(
        "{}",
        if n > 0 {
            "Positive Number."
        } else {
            "Negative Number"
        }
    );

    let value = if n > (25 / 3) { "greater." } else { "lesser." };
    println!("{}", value);
}
