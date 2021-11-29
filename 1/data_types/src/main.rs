fn main() {
    let weird_face = '😒';
    println!("{}", weird_face);

    let tup: (i32, f64, char) = (500, 3.14, '💕');
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 300, 12, 29];
    println!("{}", arr[2]);
}
