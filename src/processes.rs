use serde_json::{Map, value::Value};

use super::to_do::{
    ItemTypes,
    task_structs::{done::Done, pending::Pending},
    task_traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};

fn process_pending(item: Pending, command: String, state: &Map<String, Value>, file_name: &str) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status.to_string(),
            &mut state,
            file_name,
        ),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state, file_name),
        _ => println!("command: {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>, file_name: &str) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state, file_name),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state, file_name),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_input(
    item: ItemTypes,
    command: String,
    state: &Map<String, Value>,
    file_name: &str,
) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state, file_name),
        ItemTypes::Done(item) => process_done(item, command, state, file_name),
    }
}
