pub mod task_enum;
pub mod task_structs;
pub mod task_traits;

use task_enum::TaskStatus;
use task_structs::done::Done;
use task_structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
    }
}
