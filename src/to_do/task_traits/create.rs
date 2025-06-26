use serde_json::{Map, json, value::Value};

use crate::task_state::write_to_file;

pub trait Create {
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>, file_name: &str) {
        state.insert(title.to_string(), json!(status.to_lowercase()));
        write_to_file(file_name, state);
        println!("\n\n{} is being created\n\n", title);
    }
}
