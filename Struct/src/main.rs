// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("vijaysamrath62@mail.com"),
//         username: String::from("vijay123"),
//         active: true,
//         sign_in_count: 1
//     };

//     let name = user1.username;
//     user1.username = String::from("samrath321"),

//     let user2 = build_user(
//         String::from("kiran456@mail.com"),
//         String::from("kiran456")
//     );

//     let user3 = User {
//         email: String::from("james@mail.com"),
//         username: String::from("james123"),
//         ..User2
//     };
// }

// fn main() {

//     struct Color(i32, i32, i32);

//     struct Point(i32, i32, i32);
// }

// fn build_user(email: String, username: String) -> User{
//     User {email,
//     username,
//     active: true,
//     sign_in_count: 1,
//     }
// }

// fn main() {
//     let width1= 30;
//     let height1= 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

// fn main() {
//     let rect = (30, 50);

//     println!(
//                 "The area of the rectangle is {} square pixels.",
//                 area(rect)
//             )
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50, 
//     };

//     dbg!(&rect1);
// }

/* Methods */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
