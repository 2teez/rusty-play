#[allow(dead_code, unused)]

fn main() {
    let a = 7;
    let mut a_ref = &a;
    let mut a_box: Box<i32> = Box::new(*a_ref);

    println!("VALUE: {}, REF: {}, BOX: {}", a, a_ref, a_box);

    let c = (a + 2);
    a_ref = &c;
    a_box = Box::new(a + 2);
    println!("VALUE: {}, REF: {}, BOX: {}", a, a_ref, a_box);
}
