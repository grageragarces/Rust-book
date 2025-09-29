// Program example using structs
// takes the width and height of a rectangle specified in pixels & calculates the area of the rectangle

// // fn main() {
// //     let width1 = 30;
// //     let height1 = 50;

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(width1, height1)
// //     );
// // }

// // fn area(width: u32, height: u32) -> u32 {
// //     width * height
// // }

// // Equivalent to:

// // fn main() {
// //     let rect1 = (30, 50);

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(rect1)
// //     );
// // }

// // fn area(dimensions: (u32, u32)) -> u32 {
// //     dimensions.0 * dimensions.1
// // }

// // Equivalent to: + printing functionality struct

// #[derive(Debug)] // funtionality to print debugging info
// struct Rectangle {
//     // structures provide meaning to data
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
//     // :? -> shows everything in one line
//     // :#? -> shows everything in separate lines (easier to read for big things)

//     // can also choose between 
//     //dbg! : macro prints to the standard error console stream (stderr), takes ownership of an expression (see commented example bellow)
//     //println! :prints to the standard output console stream (stdout), takes a reference of an expression
// }


// // fn main() {
// //     let scale = 2;
// //     let rect1 = Rectangle {
// //         width: dbg!(30 * scale), // dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there
// //         height: 50,
// //     };

// //     dbg!(&rect1);
// // }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Alt 4 rectangle:

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

