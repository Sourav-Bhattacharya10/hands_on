use std::fmt::Display;

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::DONE => write!(f, "DONE"),
            TaskStatus::PENDING => write!(f, "PENDING"),
        }
    }
}
