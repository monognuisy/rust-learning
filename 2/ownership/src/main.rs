fn main() {
    let s = String::from("wow!");

    takes_ownership(s);  // 이러면 s의 소유권은 move됨! 이 이후에 사용할 수 없음!

    let x = 10;

    makes_copy(x);

    let s1 = get_hello();
    println!("{}", s1);

    let s2 = change_ownership(s1);  // s1이 s2로 move됨.
    println!("{}", s2); 
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(integer: i32) {
    println!("{}", integer);
}

fn get_hello() -> String {
    let temp_string = String::from("hello");
    temp_string
}

fn change_ownership(str: String) -> String {
    str
}
