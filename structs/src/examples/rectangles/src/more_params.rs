impl Rectangle { //impl -> every function in the struct is an associated function
    // associated funtions can be called : Rectangle::area()
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can functions within structs
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//NOTE: same struct can have multiple disjoint imp blocks defining various things within it

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}