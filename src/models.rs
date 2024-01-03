use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskRequest {
    pub kind: String,
    pub context: String 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub kind: String,
    pub context: String 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskStatus {
    pub id: String,
    pub found: bool,
    pub status: String
}
