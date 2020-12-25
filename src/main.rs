#[macro_use]
extern crate rustdart;

fn main() {
    println!("Hello, world!");

    // #[gen_dart]
    // fn test() {}

    #[gen_dart]
    struct MyStructTest {
        id: i32,
        my_string: String,
        my_double: f32
    }

    // #[gen_dart]
    // struct AnotherStructTest {
    //     my_int: i32,
    //     my_string: String,
    //     my_double: f32
    // }

    

    // println!("{:?}", test());
}
