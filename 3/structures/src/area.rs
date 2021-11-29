#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }
}

fn main() {
  let rect_tuple = (50, 30);
  let rect_struct = Rectangle {
    width: 40,
    height: 20,
  };

  println!(
    "The area of tuple rectangle is: {}",
    get_area_tuple(rect_tuple)
  );
  println!(
    "The area of struct rectangle is: {}",
    get_area_struct(&rect_struct)
  );

  print_rectangle(rect_struct);
}

fn get_area_tuple(rectangle: (u32, u32)) -> u32 {
  rectangle.0 * rectangle.1
}

fn get_area_struct(rectangle: &Rectangle) -> u32 {
  rectangle.height * rectangle.width
}

fn print_rectangle(rectangle: Rectangle) {
  println!("{:#?}", rectangle);
}
