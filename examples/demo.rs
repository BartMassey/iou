use std::collections::HashSet;

use iou::Iou;

fn strchars(s: String) -> HashSet<char> {
    s.chars().collect()
}

fn main() {
    let hello_chars = Iou::new("hello".to_string(), strchars);

    let h = hello_chars.get();
    println!("{:?}", h);

    let h = hello_chars.get_mut();
    h.remove(&'l');

    let r1 = hello_chars.get();
    println!("{:?}", r1);
    let r2 = hello_chars.get();
    println!("{:?}", r1);
    println!("{:?}", r2);

    let h = hello_chars.unwrap();
    println!("{:?}", h);
}
