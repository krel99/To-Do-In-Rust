use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone, Eq, Debug)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TaskStatus::DONE => match other {
                &TaskStatus::DONE => return true,
                &TaskStatus::PENDING => false,
            },
            TaskStatus::PENDING => match other {
                &TaskStatus::DONE => return false,
                &TaskStatus::PENDING => true,
            },
        }
    }
}
