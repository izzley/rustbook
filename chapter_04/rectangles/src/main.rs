// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectanlge is {} square pixels",
//         area(width1, height1)    
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect: (u32, u32) = (30, 50);

//     println!(
//         "The area of the rectanlge is {} square pixels",
//         area(rect)    
//     )
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// #############################


// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect: Rectangle = Rectangle {
//         width: 30,
//         height: 50
//     };

//     println!(
//         "The area of the rectanlge is {} square pixels",
//         area(&rect)    
//     )
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// ##############################################

// This is a trait. This allows the compiler to provide basic implementation of debug trait

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect: Rectangle  = Rectangle {
//         height: 50,
//         width: 30
//     };

//     println!("rect: {:#?}", rect);

//     println!(
//         "The area of the rectanlge is {} square pixels",
//         area(rect)    
//     )
// }

// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


// this is best practice as it specifies area method on Rectangle instance
impl Rectangle {
    fn area(&self,) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area_is_greater(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rect: Rectangle = dbg!(Rectangle {
        height: 50,
        width: 30
    });

    let rect1: Rectangle = Rectangle {
        height: 40,
        width: 20
    };

    let rect2: Rectangle = Rectangle {
        height: 60,
        width: 40
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect area greater than rect1: {}", rect.area_is_greater(&rect1));

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect area greater than rect2: {}", rect.area_is_greater(&rect2));




    println!("rect: {:#?}", rect);

    println!(
        "Rectangle area: {}\nRectangle1 area: {}\nRectangle2 area: {}",
        rect.area(),
        rect1.area(),
        rect2.area()
    );
}