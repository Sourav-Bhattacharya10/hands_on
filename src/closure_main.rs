// closures

// Method1
// fn add_doubles(closure: fn(i32) -> i32, one: i32, two: i32) -> i32 {
//     closure(one) + closure(two)
// }

// Method2
fn add_doubles(closure: Box<dyn Fn(i32) -> i32>, one: i32, two: i32) -> i32 {
    closure(one) + closure(two)
}

fn main() {
    // let another_str = "case";
    // let test_closure = |string_input: &str| {
    //     println!("{} {}", string_input, another_str);
    // };
    // test_closure("test");
    // println!("{}", another_str);

    // Method1
    // let closure1 = |num: i32| num * 2;
    // println!("{}", add_doubles(closure1, 2, 3));

    // Method2
    let one = 2;
    let closure1 = move |num: i32| num * one;
    println!("{}", add_doubles(Box::new(closure1), 2, 3));
}
