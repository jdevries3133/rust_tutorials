fn pointer_access() {
    let v = vec![1, 2, 3, 4, 5];

    let third_item: &i32 = &v[2];
    println!("third element is: {}", third_item);

    match v.get(2) {
        Some(thing) => println!("the third element is: {}", thing),
        None => println!("nothing is there"),
    }

    // if the index changes from 2 to a value that is not in the vector, the
    // thread will panic because of the `expect`
    println!("unsafe third element is: {}", v.get(2).expect("fook"));
}


fn take_readonly_ptr(s: &String) -> String {
    println!("I can read it: {}", s);

    // this is not allowed because s is immutable
    // s.push_str("illegal");

    // if I make a mutable copy, I can modify this, though
    let mut z = s.clone();
    z.push_str(" now it's legal to change the input");

    z
}


fn take_mut_ptr(s: &mut String) {
    println!("I can change this, I'll add extra stuff: {}", s);
    s.push_str("... extra stuff");
    println!("It has been changed: {}", s);
}


fn take_ownership(s: String) {
    println!("I own this: {}", s);
}


fn ownership() -> String {
    let x = String::from("hi");
    let y = x;


    // there will be a panic if this memory access is out of bounds
    let b = &y[0..1];

    // cannot borrow y as mutable because it's not declared as mutable
    let mut a = y.clone();
    println!("slice of a: {}", b);
    take_mut_ptr(&mut a);
    println!("a after mut ptr pass: {}", a);

    // this is also not OK. Passing y by value forefits ownership
    // take_ownership(y);
    println!("`y` before passing ptr: {}", y);
    let result = take_readonly_ptr(&y); // this is ok
    println!(
        "result is an internally copied and modified version of &y: \"{}\"",
        result
    );

    // we can give y away after we copy it, and still return it
    let z = y.clone();
    take_ownership(y);

    // this will cause an error, because it's a borrow after move. It won't
    // compile
    // println!("x is ded {}", x);

    return z;
}

fn main() {
    pointer_access();

    let x = ownership();
    println!("return value: {}", x);
}
