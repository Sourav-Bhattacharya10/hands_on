use serde_json::{Map, json, value::Value};

use super::super::task_enum::TaskStatus;
use crate::task_state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>, file_name: &str) {
        state.insert(
            title.to_string(),
            json!(TaskStatus::DONE.to_string().to_lowercase()),
        );
        write_to_file(file_name, state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>, file_name: &str) {
        state.insert(
            title.to_string(),
            json!(TaskStatus::PENDING.to_string().to_lowercase()),
        );
        write_to_file(file_name, state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}
