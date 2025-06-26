use serde_json::{Map, value::Value};

use crate::task_state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>, file_name: &str) {
        state.remove(title);
        write_to_file(file_name, state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
