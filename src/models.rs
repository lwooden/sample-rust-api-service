use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    id: i32,
    name: String,
    base_experience: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    verified: bool,
    sent_count: i32
}

#[derive(Deserialize)]
pub struct Term {
    pub term: String,
}

#[derive(Deserialize)]
pub struct Sum {
    pub val_1: i32,
    pub val_2: i32,
}

