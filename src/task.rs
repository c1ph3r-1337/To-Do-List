use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub date: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, date: String) -> Self {
        Self {
            id,
            title,
            date,
            completed: false,
        }
    }
}