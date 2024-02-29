use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    Open,
    Doing,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "OPEN",
            Self::Doing => "DOING",
            Self::Done => "DONE",
        }
    }

		pub fn from_str(s: &str) -> Option<Self> {
			match s {
					"OPEN" => Some(Self::Open),
					"DOING" => Some(Self::Doing),
					"DONE" => Some(Self::Done),
					_ => None,
			}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub removed: bool,
}

impl Task {
    pub fn new(t: String, d: String) -> Self {
        Self {
            id: None,
            title: t,
            description: d,
            removed: false,
            status: Status::Open,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}
