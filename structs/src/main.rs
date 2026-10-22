struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
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

    fn can_hold (&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width:size,
            height:size
        }
    }
}

fn main() {
    // Creating an instance of the User struct
    let user1 = User {
        active: true,
        username: String::from("someusername23"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(3);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_two(rect1)
    // );
    // println!("rect2 is {rect2:#?}")
    // dbg!(&rect2);


    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));
    println!("The square is {square:?}");

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_two(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_three(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
