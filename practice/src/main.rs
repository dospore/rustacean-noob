#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        rectangle.height <= self.height && rectangle.width <= self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };
    let rectangle2 = Rectangle {
        width: 20,
        height: 30
    };
    let rectangle3 = Rectangle::square(20);

    let text = {
        if rectangle.can_hold(&rectangle2) {
            String::from("can hold")
        }
        else {
            String::from("can not hold")
        }
    };

    let text2 = {
        if rectangle.can_hold(&rectangle3) {
            String::from("can hold")
        }
        else {
            String::from("can not hold")
        }
    };

    println!(
        "Rectangle {:#?} {} {:#?}",
        rectangle,
        text,
        rectangle2
    );

    println!(
        "Rectangle {:#?} {} {:#?}",
        rectangle,
        text2,
        rectangle3
    );



    println!(
        "The area of the {:#?} is {} square pixels.",
        rectangle,
        rectangle.area()
    );
}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// shit code
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }