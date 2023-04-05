use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    Good,
    Bad,
    Ugly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u16,
    pub gender: Gender,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Alice".into(),
            age: 32,
            gender: Gender::Female,
        }
    }
}

pub type Sequence = u64;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    // Requests
    GetPerson { sequence: Sequence },
    SetPerson { sequence: Sequence, person: Person },
    GetState { sequence: Sequence },
    // Responses
    State { sequence: Sequence, state: State },
    Person { sequence: Sequence, person: Person },
}
