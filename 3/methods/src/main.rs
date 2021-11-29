#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
  
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn print_info(&self) {
        println!("{:#?}", self);
    }

    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let s1 = Rectangle::square(10);
    s1.print_info();
    println!("area: {}", s1.get_area());
}