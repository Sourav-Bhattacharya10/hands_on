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

impl TaskStatus {
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "done" => Self::DONE,
            "pending" => Self::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}
