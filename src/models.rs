use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CatFact { 
    // status: Status,
    _id: String,
    __v: i32,
    text: String,
    updatedAt: String,
    createdAt: String,
    deleted: bool,
    user: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    verified: bool,
    sentCount: i32
}


