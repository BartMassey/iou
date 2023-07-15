use std::collections::HashSet;

use iou::Iou;

fn strchars(s: String) -> HashSet<char> {
    s.chars().collect()
}

fn main() {
    let hello_chars = Iou::new("hello".to_string(), strchars);

    let h = hello_chars.borrow();
    println!("{:?}", h);
    drop(h);

    let mut h = hello_chars.borrow_mut();
    h.remove(&'l');
    drop(h);

    let r1 = hello_chars.borrow();
    println!("{:?}", r1);
    let r2 = hello_chars.borrow();
    println!("{:?}", r2);
    drop(r1);
    drop(r2);

    let h = hello_chars.unwrap();
    println!("{:?}", h);
}
