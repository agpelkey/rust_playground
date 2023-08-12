
/*
 * First case using basic data types
fn main() {

    let width = 30;
    let height = 50;

    println!("The are of the rectangle is {} square pixels.", area(width, height));

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/


/*
 * Second case now using tuples as opposed to just basic data types
fn main() {

    let rect1 = (10, 10);

    println!("The are of the rectangle is {}", area(rect1))
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 
}
*/

fn main() {

    let rect1 = Rectangle::new_rectangle(90, 90);
    let rect2 = Rectangle::new_rectangle(50, 60);
    let rect3 = Rectangle::new_rectangle(100, 90);

    //println!("the area of the rectangle is {}", rect1.area());
    //println!("{:?}", rect1);
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn new_rectangle(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

/*
fn new_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle {
        width,
        height,
    }
}
*/

//fn area(rectangle: &Rectangle) -> u32 {
    //rectangle.width * rectangle.height
//}
