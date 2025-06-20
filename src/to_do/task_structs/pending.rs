use super::super::task_enum::TaskStatus;
use super::super::task_traits::{create::Create, edit::Edit, get::Get};
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        Self {
            super_struct: Base {
                title: String::from(input_title),
                status: TaskStatus::PENDING,
            },
        }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
