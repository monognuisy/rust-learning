struct User {
    username: String,
    age: u32,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("sungmin"),
        age: 20,
        active: true,
    };

    print_user(&user1);

    let user2 = build_user(String::from("guest"), 10);
    print_user(&user2);

    let user1_copy = User {
        ..user1
    };
    print_user(&user1_copy);

}

fn print_user(user: &User) {
    println!("username: {}", user.username);
    println!("age: {}", user.age);
    println!("active: {}", user.active);
} 

fn build_user(username: String, age: u32) -> User {
    User {
        username,
        age,
        active: true,
    }
}