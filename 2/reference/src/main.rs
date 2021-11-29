fn main() {
    let mut s1 = String::from("nice!");
    push_hello(&mut s1);
    let len = calculate_length(&s1);

    println!("length of {}: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn push_hello(s: &mut String) {
    s.push_str(" hello!");
}
