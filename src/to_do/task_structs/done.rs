use super::super::task_enum::TaskStatus;
use super::super::task_traits::{delete::Delete, edit::Edit, get::Get};
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        Self {
            super_struct: Base {
                title: String::from(input_title),
                status: TaskStatus::DONE,
            },
        }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
