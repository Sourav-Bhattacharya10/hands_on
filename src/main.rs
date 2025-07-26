// mod reverse_string;
// mod factorial;
// mod fibonacci;
// mod balanced_array;
// mod to_uppercase;
// mod two_pointer;
// mod common_prefix;
// mod processes;
// mod task_state;
// mod to_do;
// mod task_state;
// mod to_do;
// mod sliding_window;

// use serde_json::{Map, value::Value};
// use std::env;

// use factorial::factorial;
// use fibonacci::{Fibonacci, old_fibonacci};
// use reverse_string::reverse_string;
// use balanced_array::is_balanced_array;
// use to_uppercase::to_uppercase;
// use two_pointer::two_pointer_technique_for_rain_water_trap;
// use processes::process_input;
// use task_state::read_file;
// use to_do::{task_enum::TaskStatus, to_do_factory};
// use task_state::{read_file, write_to_file};
// use to_do::{
//     ItemTypes,
//     task_enum::TaskStatus,
//     task_traits::{delete::Delete, edit::Edit, get::Get},
//     to_do_factory,
// };
// use common_prefix::find_common_prefix;
// use sliding_window::{
//     // basic_sliding_window,
//     // longest_subarray,
//     length_of_longest_substring,
// };

// use common_prefix::find_common_prefix;

fn main() {
    //     // let text = "sourav";
    //     // println!("Reversed String: {:?}", reverse_string(text));

    //     // let n: u32 = 5;
    //     // println!("Factorial of {} is {}", n, factorial(n));

    // let n: u32 = 10;
    // println!("Fibonacci of {}th term is {}", n, old_fibonacci(n));

    // let fibs = Fibonacci::new();
    // println!("New Fibonacci of {}th term is {}", n, fibs.nth_term(n));
    //     // println!("[ -2, 3, 2, -3 ] → {}", is_balanced_array(vec![ -2, 3, 2, -3 ]));
    //     // println!("[ 1, 1, -1, -1 ] → {}", is_balanced_array(vec![ 1, 1, -1, -1 ]));
    //     // println!("[ 1, 1, -1 ] → {}", is_balanced_array(vec![ 1, 1, -1 ]));
    //     // println!("[ -2, 3, 2, -3, 0, 5, -5 ] → {}", is_balanced_array(vec![ -2, 3, 2, -3, 0, 5, -5 ]));
    //     // println!("[ 1, 2, -3 ] → {}", is_balanced_array(vec![ 1, 2, -3 ]));
    //     // println!("[ -3, -2, -3, -2, 4, 1, 4, 1, 3, 2, -4, -1 ] → {}", is_balanced_array(vec![ -3, -2, -3, -2, 4, 1, 4, 1, 3, 2, -4, -1 ]));

    //     // println!("to uppercase : {}", to_uppercase(String::from(text)));

    //     // println!(
    //     //     "[ 10, 20, 35, 50 ] → {:?}",
    //     //     two_pointer_technique_for_two_sum(vec![10, 20, 35, 50], 70i8)
    //     // );
    //     // println!(
    //     //     "[ 10, 20, 30 ] → {:?}",
    //     //     two_pointer_technique_for_two_sum(vec![10, 20, 30], 70i8)
    //     // );
    //     // println!(
    //     //     "[ 1, 4, 6, 8, 10, 45 ] → {:?}",
    //     //     two_pointer_technique_for_three_sum(vec![1, 4, 6, 8, 10, 45], 13i8)
    //     // );
    //     // println!(
    //     //     "[ 3, 0, 1, 0, 4, 0, 2 ] → {:?}",
    //     //     two_pointer_technique_for_rain_water_trap(vec![3, 0, 1, 0, 4, 0, 2])
    //     // );

    //     // let words_array = vec!["flower", "flow", "flood"];
    //     // println!("Common Prefix: {}", find_common_prefix(words_array));

    // let array: Vec<i8> = vec![5, 2, -1, 0, 3];
    // println!(
    //     "The max sum of any window is {}",
    //     basic_sliding_window(array, 3)
    // );

    // let array: Vec<i8> = vec![-5, 8, -14, 2, 4, 12];
    // println!("The longest subarray is {}", longest_subarray(array, -5));

    // let array: Vec<i8> = vec![10, -10, 20, 30];
    // println!("The longest subarray is {}", longest_subarray(array, 5));

    // let array = "abcabcbb";
    // println!(
    //     "The length of longest subarray is {}",
    //     length_of_longest_substring(array)
    // );

    //     let args: Vec<String> = env::args().collect();
    //     let command: &String = &args[1];
    //     let title: &String = &args[2];
    //     let file_name = "./state.json";
    //     let state: Map<String, Value> = read_file(file_name);
    //     let status: String;
    //     match &state.get(*&title) {
    //         Some(result) => {
    //             status = result.to_string().replace('\"', "");
    //         }
    //         None => {
    //             status = "pending".to_owned();
    //         }
    //     }
    //     let item = to_do_factory(title, TaskStatus::from_string(status));
    //     process_input(item, command.to_string(), &state, file_name);
}
